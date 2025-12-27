# System Calls, File Operations, and Cargo Crate Audit Report

## Executive Summary

This audit examines the split-decls-rs codebase for system calls, file operations, cargo crate handling, and identifies issues with alias renaming and feature management.

## System Calls and Process Execution

### Process Execution Patterns
- **Command execution**: Uses `std::process::Command` extensively
- **Audit macros**: `audit_execute!` macro in audited bootstrap files
- **Shell commands**: Direct execution of `curl`, `cargo`, and other system tools

### Key Files with System Calls:
1. **workflow_executor.rs**: Core execution engine
2. **bootstrap_scanner_audited.rs**: Audited process execution
3. **bootstrap-self-apply_audited.rs**: Self-application with audit trail
4. **stateful_repl.rs**: Interactive REPL with external command execution

### System Call Audit Findings:
```rust
// Audited execution pattern
macro_rules! audit_execute {
    ($cmd:expr) => {
        let timestamp = std::time::SystemTime::now();
        println!("⚠️  PROCESS AUDIT: {:?}", timestamp);
        println!("📋 Command: {}", stringify!($cmd));
        println!("📁 PWD: {:?}", std::env::current_dir().unwrap_or_default());
        // ... execution and timing
    };
}
```

## File System Operations

### File Write Operations
- **Configuration files**: Extensive TOML file generation and modification
- **Source code generation**: Rust file creation in `src/decls/` directories
- **Backup operations**: Original file preservation with `.old` extensions
- **Workspace management**: Root `Cargo.toml` manipulation

### Critical File Operations:
1. **Cargo.toml generation**: `generate_new_cargo.rs`
2. **Workspace management**: `workspace_manager.rs`
3. **File backup**: `backup_original_cargo.rs`
4. **Directory copying**: `copy_dir_recursive.rs`

### File Write Audit Pattern:
```rust
macro_rules! audit_write {
    ($path:expr, $content:expr) => {
        let timestamp = std::time::SystemTime::now();
        println!("⚠️  FILE WRITE AUDIT: {:?}", timestamp);
        println!("📝 Writing to: {:?}", $path);
        println!("📊 Size: {} bytes", $content.len());
        std::fs::write($path, $content)?;
    };
}
```

## Cargo Crate and Feature Handling

### Dependency Management Architecture
- **Workspace dependencies**: Centralized in root `Cargo.toml`
- **Feature propagation**: Automatic feature addition for key crates
- **Path resolution**: Complex submodule path mapping
- **Patch management**: `[patch.crates-io]` generation

### Key Components:
1. **generate_new_cargo.rs**: Core Cargo.toml generation
2. **workspace_manager.rs**: Workspace-level dependency management
3. **regen_cargo_v2.rs**: New lib-cargo based regeneration
4. **split-decls-rs.toml**: Configuration with path overrides

## Alias and Renaming Issues

### Current Problems Identified:

#### 1. Incomplete Alias Handling
**Issue**: The system partially renames features but doesn't update all usage references.

**Evidence**:
```rust
// In regen_cargo.rs - only handles specific alloc case
let fixed_content = cargo_content
    .replace("alloc]\nworkspace = true", "rustc-std-workspace-alloc]\nworkspace = true")
    .replace("[dependencies.alloc]", "[dependencies.rustc-std-workspace-alloc]")
    .replace("[dev-dependencies.alloc]", "[dev-dependencies.rustc-std-workspace-alloc]")
    .replace("[build-dependencies.alloc]", "[build-dependencies.rustc-std-workspace-alloc]");
```

#### 2. Package Alias Detection
**Current Implementation**:
```rust
// report_aliases.rs - detects but doesn't fix
if let Some(package) = table.get("package").and_then(|v| v.as_str()) {
    aliases.insert(dep_name.clone(), (package.to_string(), crate_name.to_string()));
}
```

#### 3. Feature Propagation Issues
**Problem**: Features are added to dependencies but not consistently propagated to usage sites.

**Evidence**:
```rust
// generate_new_cargo.rs - adds features but doesn't track usage
match *dep_name {
    "syn" => ensure_workspace_dependency(build_deps_table, dep_name, Some(vec!["full", "visit"])),
    "serde" => ensure_workspace_dependency(build_deps_table, dep_name, Some(vec!["derive"])),
    _ => ensure_workspace_dependency(build_deps_table, dep_name, None),
}
```

### Specific Alias Problems:

#### 1. `alloc` → `rustc-std-workspace-alloc`
- **Status**: Partially implemented
- **Issue**: Only handles Cargo.toml renaming, not Rust code usage
- **Impact**: Compilation failures when code uses `extern crate alloc`

#### 2. `aho_corasick` → `aho-corasick`
- **Status**: Path override exists but usage not updated
- **Issue**: Underscore vs hyphen inconsistency
- **Impact**: Dependency resolution failures

#### 3. `serde_json` → `json`
- **Status**: Path override exists
- **Issue**: Import statements still use old name
- **Impact**: Module not found errors

## Configuration Analysis

### split-decls-rs.toml Structure:
```toml
[crate_path_overrides]
aho_corasick = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/aho-corasick"
serde_json = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/json"
alloc = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/library/rustc-std-workspace-alloc"
```

### Issues:
1. **Inconsistent naming**: Configuration uses underscores, paths use hyphens
2. **Missing reverse mapping**: No way to map from new name back to old usage
3. **Feature inheritance**: No mechanism to propagate features through aliases

## Recommendations

### 1. Comprehensive Alias System
Create a complete alias resolution system:

```rust
pub struct AliasResolver {
    aliases: HashMap<String, AliasMapping>,
}

pub struct AliasMapping {
    old_name: String,
    new_name: String,
    path: PathBuf,
    features: Vec<String>,
    usage_patterns: Vec<String>, // Regex patterns for code updates
}
```

### 2. Code Usage Updates
Implement source code transformation for alias usage:

```rust
pub fn update_rust_code_for_aliases(
    file_path: &Path,
    aliases: &HashMap<String, AliasMapping>
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let mut updated_content = content;
    
    for (old_name, mapping) in aliases {
        // Update extern crate statements
        updated_content = updated_content.replace(
            &format!("extern crate {};", old_name),
            &format!("extern crate {} as {};", mapping.new_name, old_name)
        );
        
        // Update use statements
        updated_content = updated_content.replace(
            &format!("use {};", old_name),
            &format!("use {};", mapping.new_name)
        );
    }
    
    fs::write(file_path, updated_content)?;
    Ok(())
}
```

### 3. Feature Consistency
Implement feature propagation tracking:

```rust
pub struct FeatureTracker {
    crate_features: HashMap<String, Vec<String>>,
    feature_dependencies: HashMap<String, Vec<String>>,
}

impl FeatureTracker {
    pub fn propagate_features(&mut self, crate_name: &str) -> Vec<String> {
        // Calculate transitive feature requirements
        // Return complete feature set needed
    }
}
```

### 4. Validation System
Add comprehensive validation:

```rust
pub fn validate_alias_consistency(config: &SplitDeclsConfig) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    // Check for naming inconsistencies
    // Verify path mappings exist
    // Validate feature compatibility
    // Check for circular dependencies
    
    Ok(errors)
}
```

## Security Considerations

### File System Access
- **Unrestricted writes**: System writes to arbitrary paths
- **Backup safety**: Original files preserved but not validated
- **Path traversal**: Limited validation of output paths

### Process Execution
- **Command injection**: Limited sanitization of external commands
- **Environment exposure**: Process execution inherits full environment
- **Audit trail**: Good audit logging in place for security-sensitive operations

## Performance Impact

### File Operations
- **Bulk processing**: Processes hundreds of crates simultaneously
- **Redundant reads**: Multiple reads of same Cargo.toml files
- **Memory usage**: Large TOML structures kept in memory

### Optimization Opportunities
1. **Caching**: Cache parsed TOML files
2. **Batch operations**: Group file operations
3. **Parallel processing**: Leverage existing parallel crate processing

## Conclusion

The split-decls-rs system has a sophisticated cargo handling system but suffers from incomplete alias resolution. The main issues are:

1. **Partial alias implementation**: Only Cargo.toml renaming, not code usage
2. **Inconsistent naming**: Underscores vs hyphens in different contexts
3. **Missing feature propagation**: Features added but not consistently used
4. **No validation**: No system to verify alias consistency

The recommended solution is a comprehensive alias resolver that handles both configuration and code transformation, with proper validation and feature propagation.
