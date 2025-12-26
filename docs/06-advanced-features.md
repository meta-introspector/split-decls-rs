# Split-Decls-RS Documentation Series

## Part 6: Advanced Features & Integration

### Workspace Management System

Split-decls-rs includes sophisticated workspace management for handling large-scale Rust ecosystems.

#### Automatic Dependency Resolution

```rust
// gen_workspace tool automatically discovers dependencies
cargo run --bin gen_workspace

// Output:
// Scanning wrapped crates for missing dependencies...
// ✅ Generated workspace with 674 members and 878 dependencies
```

The system:
1. Scans all wrapped crates for `workspace = true` dependencies
2. Adds missing dependencies to root `Cargo.toml`
3. Resolves version conflicts automatically
4. Generates `[patch.crates-io]` entries for local overrides

#### Workspace Configuration

```toml
# Generated workspace structure
[workspace]
members = [
    "wrapped-addr2line",
    "wrapped-serde", 
    "wrapped-tokio",
    # ... 674 total members
]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
# ... 878 total dependencies

[patch.crates-io]
addr2line = { path = "wrapped-addr2line" }
serde = { path = "wrapped-serde" }
# Automatic local overrides
```

### Bootstrap & Self-Modification System

#### Bootstrap Process

```bash
# Full ecosystem bootstrap
make run_bootstrap

# Process:
# 1. Discover all crates in submodules/
# 2. Wrap each crate individually  
# 3. Generate workspace configuration
# 4. Resolve dependencies
# 5. Create output2/ with all wrapped crates
```

#### Self-Modification Capability

The system can modify itself:

```rust
// Bootstrap tracer captures transformation process
pub struct BootstrapTracer {
    pub trace: ExecutionTrace,
    pub rdf_graph: Vec<RdfTriple>,
    pub transformation_log: Vec<TransformationEvent>,
}

// Self-application example
let mut tracer = BootstrapTracer::new();
tracer.trace_self_modification("split-decls-rs")?;
// System wraps itself, creating wrapped-split-decls-rs
```

### Patch System Integration

#### Configuration-Driven Patching

```toml
# split-decls-rs.toml
[patches]
"error_handling_patch.rs" = ["wrapped-addr2line", "wrapped-serde"]
"async_patch.rs" = ["wrapped-tokio"]

[string_replacements]
"std::error::Error" = "anyhow::Error"
"println!" = "log::info!"

[custom_prelude_overlay]
addr2line = """
use anyhow::{Result, Context};
use log::{info, warn, error};
"""
```

#### Patch Application

```rust
// Patches applied during wrapping process
fn apply_patches(crate_name: &str, content: &str, config: &SplitDeclsConfig) -> String {
    let mut patched = content.to_string();
    
    // Apply string replacements
    for (from, to) in &config.string_replacements {
        patched = patched.replace(from, to);
    }
    
    // Apply prelude overlay
    if let Some(prelude) = config.custom_prelude_overlay.get(crate_name) {
        patched = format!("{}\n{}", prelude, patched);
    }
    
    patched
}
```

### Analysis & Introspection Tools

#### AST Statistics Collection

```rust
pub struct AstStatistics {
    pub function_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub impl_count: usize,
    pub complexity_metrics: HashMap<String, f64>,
}

// Usage:
let mut stats = AstStatistics::new();
stats.analyze_crate("wrapped-addr2line")?;
println!("Functions: {}, Structs: {}", stats.function_count, stats.struct_count);
```

#### Meta-Pattern Analysis

```rust
// Detect common patterns across wrapped crates
let mut visitor = MetaPatternVisitor::new();
visitor.analyze_workspace("output2/")?;

// Results:
// - Error handling patterns: 47 occurrences
// - Builder patterns: 23 occurrences  
// - Iterator patterns: 156 occurrences
```

### Integration with External Tools

#### Cargo Integration

```bash
# Works with standard Cargo commands
cd output2/wrapped-addr2line
cargo build    # Builds wrapped crate
cargo test     # Runs tests on wrapped code
cargo doc      # Generates docs for wrapped declarations
```

#### IDE Integration

```rust
// Language Server Protocol support
// Each wrapped declaration is a separate file = better IDE support
// - Go to definition works across wrapped boundaries
// - Autocomplete for wrapped declarations
// - Refactoring tools work on individual declarations
```

#### Debugger Integration

```bash
# GDB with symbol resolution
gdb /proc/self/exe
(gdb) info symbol 0x5e5f6da0
# Resolves to: Error in wrapped-addr2line/src/decls/Error.rs

# LLDB with source mapping
lldb /proc/self/exe  
(lldb) image lookup --address 0x5e5f6da0
# Shows: wrapped-addr2line Error type declaration
```

### Performance Optimization Features

#### Parallel Processing

```rust
// Parallel crate wrapping
use rayon::prelude::*;

crates.par_iter().for_each(|crate_path| {
    wrap_single_crate(crate_path).unwrap();
});

// Results in ~4x speedup on multi-core systems
```

#### Incremental Compilation

```rust
// Only re-wrap changed crates
fn needs_rewrapping(crate_path: &Path) -> bool {
    let source_mtime = get_mtime(&crate_path.join("src/lib.rs"));
    let wrapped_mtime = get_mtime(&output_path(crate_path));
    source_mtime > wrapped_mtime
}
```

#### Caching System

```rust
// Cache parsed ASTs and generated code
pub struct WrapperCache {
    ast_cache: HashMap<String, syn::File>,
    generated_cache: HashMap<String, String>,
}

// ~10x speedup for repeated operations
```

### Advanced Macro Features

#### Conditional Compilation Support

```rust
// Handles #[cfg] attributes properly
#[cfg(feature = "std")]
pub fn std_only_function() { }

// Generates:
// wrapped_crate_decls_std_only_function.rs (only if std feature enabled)
```

#### Generic Type Handling

```rust
// Preserves generic parameters
pub struct Context<R: Read> {
    reader: R,
}

// Generates proper generic declaration:
// struct Context<R: Read> { reader: R }
```

#### Lifetime Management

```rust
// Maintains lifetime annotations
pub fn parse<'a>(input: &'a str) -> Result<&'a str> { }

// Preserved in wrapped declaration
```

### Ecosystem Integration Points

#### Nix Integration

```nix
# Nix flake support for wrapped crates
{
  inputs.split-decls-rs.url = "github:user/split-decls-rs";
  
  outputs = { self, split-decls-rs }: {
    packages.x86_64-linux.wrapped-ecosystem = 
      split-decls-rs.lib.wrapRustEcosystem ./submodules;
  };
}
```

#### CI/CD Integration

```yaml
# GitHub Actions workflow
- name: Wrap Rust Ecosystem
  run: |
    cargo run --bin split-decls-rs -- bootstrap
    make gen_workspace
    cd output2 && cargo build --workspace

- name: Test Wrapped Code
  run: |
    make test_addr2line_module
    make analyze_terms
```

### Monitoring & Observability

#### Execution Tracing

```rust
// Trace all macro executions
pub struct ExecutionTracer {
    pub calls: Vec<MacroCall>,
    pub timing: HashMap<String, Duration>,
    pub memory_usage: HashMap<String, usize>,
}

// Usage:
let tracer = ExecutionTracer::new();
tracer.trace_execution("(call Error \"test\")")?;
```

#### Metrics Collection

```rust
// System-wide metrics
pub struct SystemMetrics {
    pub total_declarations: usize,      // 3,328
    pub wrapped_crates: usize,          // 674
    pub real_addresses: usize,          // 24
    pub hash_addresses: usize,          // 3,304
    pub execution_time: Duration,       // 200ms
    pub memory_usage: usize,           // 50MB
}
```

### Error Recovery & Resilience

#### Graceful Degradation

```rust
// System continues working even with partial failures
match wrap_crate(crate_path) {
    Ok(_) => println!("✅ Wrapped {}", crate_name),
    Err(e) => {
        eprintln!("⚠️ Failed to wrap {}: {}", crate_name, e);
        // Continue with other crates
        continue;
    }
}
```

#### Automatic Repair

```rust
// Detect and fix common issues
fn auto_repair_workspace() -> Result<()> {
    // Fix missing dependencies
    run_gen_workspace()?;
    
    // Resolve version conflicts
    resolve_dependency_conflicts()?;
    
    // Regenerate broken wrappers
    rewrap_failed_crates()?;
    
    Ok(())
}
```

### Next Steps

- **Part 7**: Learn about extending the system and contributing

---
*Advanced features enable split-decls-rs to handle real-world Rust ecosystems at scale with robust error handling and performance optimization.*
