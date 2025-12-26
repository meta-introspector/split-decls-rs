# Split-Decls-RS Documentation Series

## Part 7: Extending & Contributing

### Architecture for Extension

Split-decls-rs is designed with extensibility as a core principle. The modular architecture allows for easy extension and customization.

### Extension Points

#### 1. Custom Declaration Processors

```rust
// Implement custom processing for specific declaration types
pub trait DeclarationProcessor {
    fn can_process(&self, decl_type: &str) -> bool;
    fn process(&self, decl: &syn::Item) -> Result<String>;
    fn generate_wrapper(&self, decl: &syn::Item) -> Result<String>;
}

// Example: Custom async function processor
pub struct AsyncFunctionProcessor;

impl DeclarationProcessor for AsyncFunctionProcessor {
    fn can_process(&self, decl_type: &str) -> bool {
        decl_type == "async_function"
    }
    
    fn process(&self, decl: &syn::Item) -> Result<String> {
        // Custom async function transformation
        Ok(format!("// Async wrapper for {}", quote!(#decl)))
    }
}
```

#### 2. Address Resolution Strategies

```rust
// Custom address resolution methods
pub trait AddressResolver {
    fn resolve(&self, symbol: &str) -> Result<String>;
    fn priority(&self) -> u8; // Higher priority = tried first
}

// Example: DWARF debug info resolver
pub struct DwarfResolver {
    debug_info: gimli::DebugInfo,
}

impl AddressResolver for DwarfResolver {
    fn resolve(&self, symbol: &str) -> Result<String> {
        // Use DWARF debug information for precise addresses
        self.debug_info.lookup_symbol(symbol)
    }
    
    fn priority(&self) -> u8 { 100 } // High priority
}
```

#### 3. Evaluation Engine Extensions

```rust
// Custom expression evaluators
pub trait ExpressionEvaluator {
    fn can_evaluate(&self, expr: &str) -> bool;
    fn evaluate(&self, expr: &str, context: &EvaluationContext) -> Result<String>;
}

// Example: Mathematical expression evaluator
pub struct MathEvaluator;

impl ExpressionEvaluator for MathEvaluator {
    fn can_evaluate(&self, expr: &str) -> bool {
        expr.starts_with("(math ")
    }
    
    fn evaluate(&self, expr: &str, context: &EvaluationContext) -> Result<String> {
        // Parse and evaluate mathematical expressions
        // (math add 2 3) -> "5"
        Ok("5".to_string())
    }
}
```

### Plugin System

#### Plugin Architecture

```rust
// Plugin trait for extending functionality
pub trait SplitDeclsPlugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, config: &PluginConfig) -> Result<()>;
    fn process_crate(&self, crate_info: &CrateInfo) -> Result<()>;
    fn finalize(&self) -> Result<()>;
}

// Plugin registration
pub struct PluginRegistry {
    plugins: Vec<Box<dyn SplitDeclsPlugin>>,
}

impl PluginRegistry {
    pub fn register<P: SplitDeclsPlugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }
}
```

#### Example Plugin: Metrics Collection

```rust
pub struct MetricsPlugin {
    metrics: HashMap<String, u64>,
}

impl SplitDeclsPlugin for MetricsPlugin {
    fn name(&self) -> &str { "metrics-collector" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn process_crate(&self, crate_info: &CrateInfo) -> Result<()> {
        // Collect metrics during processing
        self.metrics.insert(
            format!("{}_declarations", crate_info.name),
            crate_info.declaration_count as u64
        );
        Ok(())
    }
    
    fn finalize(&self) -> Result<()> {
        // Export metrics at the end
        std::fs::write("metrics.json", serde_json::to_string(&self.metrics)?)?;
        Ok(())
    }
}
```

### Contributing Guidelines

#### Development Setup

```bash
# 1. Fork and clone the repository
git clone https://github.com/your-username/split-decls-rs.git
cd split-decls-rs

# 2. Set up development environment
rustup update stable
cargo install cargo-watch cargo-expand

# 3. Run tests to verify setup
cargo test
make test_addr2line_module

# 4. Set up pre-commit hooks
cp scripts/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit
```

#### Code Style & Standards

```rust
// Follow Rust standard formatting
cargo fmt

// Ensure clippy compliance
cargo clippy -- -D warnings

// Document all public APIs
/// Maps a declaration name to its memory address
/// 
/// # Arguments
/// * `decl_name` - The name of the declaration to resolve
/// 
/// # Returns
/// * `String` - The hexadecimal address (real or hash-based)
/// 
/// # Examples
/// ```
/// let addr = decl2addr!("Error");
/// assert!(addr.starts_with("0x"));
/// ```
pub fn resolve_declaration_address(decl_name: &str) -> String {
    // Implementation
}
```

#### Testing Requirements

```rust
// Unit tests for all new functionality
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_declaration_wrapping() {
        let input = "pub struct Test { field: u32 }";
        let result = wrap_declaration(input).unwrap();
        assert!(result.contains("struct Test"));
    }
    
    #[test]
    fn test_address_resolution() {
        let addr = resolve_address("test_symbol");
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 10); // 0x + 8 hex digits
    }
}

// Integration tests for end-to-end workflows
#[test]
fn test_full_wrapping_workflow() {
    let temp_dir = tempfile::tempdir().unwrap();
    create_test_crate(&temp_dir);
    
    let result = wrap_single_crate(temp_dir.path()).unwrap();
    assert!(result.declarations.len() > 0);
    
    // Verify generated files exist
    assert!(temp_dir.path().join("src/decls").exists());
}
```

### Adding New Features

#### Feature Development Workflow

1. **Design Phase**
   ```rust
   // Create RFC document in docs/rfcs/
   // Example: docs/rfcs/0001-custom-evaluators.md
   ```

2. **Implementation Phase**
   ```rust
   // Create feature branch
   git checkout -b feature/custom-evaluators
   
   // Implement core functionality
   // Add comprehensive tests
   // Update documentation
   ```

3. **Integration Phase**
   ```rust
   // Add integration tests
   // Update make targets if needed
   // Verify backward compatibility
   ```

#### Example: Adding WebAssembly Support

```rust
// 1. Define the feature interface
pub trait WasmExporter {
    fn export_to_wasm(&self, declarations: &[MacroDeclaration]) -> Result<Vec<u8>>;
}

// 2. Implement the feature
pub struct WasmExporter {
    module_builder: wasmtime::Module,
}

impl WasmExporter for WasmExporter {
    fn export_to_wasm(&self, declarations: &[MacroDeclaration]) -> Result<Vec<u8>> {
        // Convert declarations to WASM module
        let mut module = wasmtime::Module::new();
        
        for decl in declarations {
            match decl.declaration_type.as_str() {
                "function" => module.add_function(&decl.name, &decl.content)?,
                "struct" => module.add_type(&decl.name, &decl.content)?,
                _ => {} // Skip unsupported types
            }
        }
        
        module.compile()
    }
}

// 3. Add CLI integration
#[derive(Parser)]
struct ExportWasmArgs {
    #[arg(long)]
    output: PathBuf,
    
    #[arg(long)]
    declarations: Vec<String>,
}

// 4. Add make target
// export_wasm:
//     cargo run --bin export_wasm -- --output module.wasm
```

### Documentation Standards

#### API Documentation

```rust
/// # Split-Decls-RS Core API
/// 
/// This module provides the core functionality for wrapping Rust declarations
/// into addressable, callable modules.
/// 
/// ## Quick Start
/// 
/// ```rust
/// use split_decls_rs::*;
/// 
/// // Wrap a single crate
/// let result = wrap_single_crate("../my-crate")?;
/// println!("Wrapped {} declarations", result.len());
/// 
/// // Map declarations to addresses
/// let all_decls = alldecls!();
/// for (name, addr) in all_decls {
///     println!("{} → {}", name, addr.address);
/// }
/// ```
pub mod core_api {
    // Implementation
}
```

#### Example Documentation

```rust
/// # Examples
/// 
/// ## Basic Wrapping
/// 
/// ```rust
/// # use split_decls_rs::*;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let wrapped = wrap_single_crate("../addr2line")?;
/// assert!(wrapped.declarations.len() > 0);
/// # Ok(())
/// # }
/// ```
/// 
/// ## Address Resolution
/// 
/// ```rust
/// # use split_decls_rs::*;
/// let addr = decl2addr!("Error");
/// assert!(addr.starts_with("0x"));
/// ```
```

### Performance Considerations

#### Benchmarking New Features

```rust
// Use criterion for performance testing
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_declaration_wrapping(c: &mut Criterion) {
    let test_code = include_str!("../test_data/large_crate.rs");
    
    c.bench_function("wrap_declarations", |b| {
        b.iter(|| {
            wrap_declarations(black_box(test_code))
        })
    });
}

criterion_group!(benches, bench_declaration_wrapping);
criterion_main!(benches);
```

#### Memory Usage Guidelines

```rust
// Monitor memory usage for large codebases
fn process_large_codebase() -> Result<()> {
    let initial_memory = get_memory_usage();
    
    // Process declarations in chunks to avoid memory spikes
    for chunk in declarations.chunks(1000) {
        process_declaration_chunk(chunk)?;
        
        // Check memory usage periodically
        let current_memory = get_memory_usage();
        if current_memory - initial_memory > MAX_MEMORY_INCREASE {
            // Trigger garbage collection or cleanup
            cleanup_temporary_data()?;
        }
    }
    
    Ok(())
}
```

### Release Process

#### Version Management

```toml
# Cargo.toml versioning
[package]
name = "split-decls-rs"
version = "0.2.0"  # Follow semantic versioning

# Update CHANGELOG.md for each release
## [0.2.0] - 2024-12-26
### Added
- WebAssembly export functionality
- Custom evaluator plugin system
- Performance improvements for large codebases

### Changed
- Address resolution now supports DWARF debug info
- Improved error messages for wrapping failures

### Fixed
- Memory leak in declaration processing
- Race condition in parallel wrapping
```

#### CI/CD Pipeline

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run comprehensive tests
        run: |
          cargo test --all-features
          make test_addr2line_module
          make analyze_terms
  
  release:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Create GitHub release
        uses: actions/create-release@v1
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
```

### Community & Support

#### Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: General questions and community support
- **Documentation**: Comprehensive guides and API reference
- **Examples**: Real-world usage examples in `examples/`

#### Contributing Areas

1. **Core Engine**: AST processing, address resolution, evaluation
2. **Tooling**: CLI improvements, IDE integration, debugging tools
3. **Ecosystem**: Plugin development, language bindings, integrations
4. **Documentation**: Tutorials, guides, API documentation
5. **Testing**: Test coverage, performance benchmarks, edge cases

---

*Split-decls-rs thrives on community contributions. Whether you're fixing bugs, adding features, or improving documentation, your contributions help advance the state of Rust tooling.*

---

## Documentation Series Complete

This 7-part documentation series covers the complete split-decls-rs system:

1. **System Overview** - Architecture and core concepts
2. **Installation & Quick Start** - Getting up and running
3. **Wrapping System** - How code transformation works
4. **Address Mapping** - Memory layout and address resolution
5. **Macro System** - Evaluation engine and lisp-like expressions
6. **Advanced Features** - Workspace management and integrations
7. **Extending & Contributing** - Development and community guidelines

The system represents a new paradigm in Rust tooling - making every piece of code addressable, callable, and transformable.
