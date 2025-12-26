# Workspace-Related Code Documentation

## Root Workspace Generation (Core Functions)

### **lib-cargo/src/workspace.rs** - PRIMARY WORKSPACE GENERATOR
- **`generate_workspace_toml()`** - **ROOT FUNCTION** for workspace Cargo.toml generation
- **`collect_workspace_members()`** - Scans for wrapped-* directories 
- **`collect_all_workspace_dependencies()`** - **KEY FUNCTION** - Collects deps from all crates, converts to paths

### **src/bin/regen_cargo_v2.rs** - MAIN CLI TOOL
- **PRIMARY ENTRY POINT** for workspace regeneration
- Calls `collect_workspace_members()` and `collect_all_workspace_dependencies()`
- Uses `generate_workspace_toml()` to write final workspace Cargo.toml
- Loads split-decls-rs.toml config for package aliases

## Common Dependency Handling Patterns

### **Pattern 1: Workspace Dependency Collection**
```rust
// Found in: lib-cargo/src/workspace.rs, src/wrapped_workspace_handlers/utils.rs
collect_all_workspace_dependencies() -> HashMap<String, Value>
collect_and_format_workspace_dependencies() -> HashMap<String, Value>
```

### **Pattern 2: Path Generation**
```rust
// Common pattern across multiple files:
let wrapped_path = format!("./wrapped-{}", name);
workspace_entry.insert("path".to_string(), Value::String(wrapped_path));
```

### **Pattern 3: Workspace Structure**
```rust
// Found in: cargo-toml-generator-types, split-decls-types
pub struct Workspace {
    pub members: Vec<String>,
    pub workspace_dependencies: HashMap<String, Dependency>,
}
```

## Duplicate/Redundant Code Identified

### **DUPLICATE WORKSPACE MANAGERS**
- `src/workspace_manager.rs` 
- `workspace-merge/src/workspace_manager.rs`
- **SAME FUNCTION**: `manage_workspace_dependencies()` - appears in both files

### **DUPLICATE WORKSPACE GENERATION**
- `src/generate_new_workspace.rs` - Basic workspace generation
- `src/generate_wrapped_workspace.rs` - Wrapped workspace generation  
- `src/bin/gen_workspace.rs` - CLI workspace generation
- **OVERLAP**: All generate workspace Cargo.toml files

### **DUPLICATE DEPENDENCY COLLECTION**
- `lib-cargo/src/workspace.rs::collect_all_workspace_dependencies()`
- `src/wrapped_workspace_handlers/utils.rs::collect_and_format_workspace_dependencies()`
- **SIMILAR LOGIC**: Both collect and format workspace dependencies

## Root Cause Analysis

**MAIN ISSUE**: The `lib-cargo/src/workspace.rs::collect_all_workspace_dependencies()` was generating:
```rust
workspace_entry.insert("workspace".to_string(), Value::Boolean(true)); // WRONG - circular reference
```

**SHOULD BE**:
```rust
workspace_entry.insert("path".to_string(), Value::String(format!("./wrapped-{}", name))); // CORRECT
```

## Consolidation Recommendations

1. **Use lib-cargo as single source of truth** for workspace generation
2. **Remove duplicate workspace managers** - keep only `src/workspace_manager.rs`
3. **Consolidate dependency collection** into `lib-cargo/src/workspace.rs`
4. **Ensure all workspace generation uses generated headers** from special_print.rs
