# Dependency Resolution Process Documentation

## Problem
The split-decls-rs system generates wrapped Rust crates in `output2/` workspace, but dependency conflicts prevent compilation due to:
1. Mixed dependency specifications (string vs table vs workspace)
2. Incorrect alias mappings 
3. Missing workspace dependencies
4. Outdated paths after crate relocations

## Solution: Unified Iterative Process

### Core Components

#### 1. Dependency Analysis Tool (`regen_cargo_v2 --analyze-only`)
- Analyzes all Cargo.toml files in output2 workspace
- Reports dependency conflicts with detailed statistics
- Shows exactly which dependencies have conflicting specifications
- Example output: `serde` appears as both `version="1.0"` and `{workspace=true}`

#### 2. Path Validation (`update_split_decls_config`)
- Automatically validates all paths in `split-decls-rs.toml`
- Fixes moved crates (e.g., `submodules/rust/compiler/` → `crates/`)
- Reports and corrects invalid paths
- Searches common locations: `crates/`, `submodules/`, `tools/`

#### 3. Alias Normalization (`normalize_syn_dependencies`)
- Converts ALL dependencies to `workspace = true` format
- Handles alias renaming (e.g., `core` → `rustc-std-workspace-core`)
- Removes conflicting path specifications
- Ensures consistent dependency resolution

#### 4. Workspace Dependency Auto-Addition
- Automatically adds missing dependencies to workspace
- Uses analysis results to populate workspace dependencies
- Points to correct submodule paths

### Iterative Process

```bash
# 1. Analyze current state
cargo run --bin regen_cargo_v2 -- --analyze-only

# 2. Update and validate paths  
cargo run --bin update_split_decls_config -- --split-decls-config-path split-decls-rs.toml

# 3. Regenerate Cargo.toml files with fixes
make regen_cargo

# 4. Test compilation
cargo check --manifest-path output2/Cargo.toml

# 5. Bootstrap more crates if needed
make run_bootstrap

# 6. Repeat until clean compilation
```

## Key Learnings

### 1. Dependency Conflicts Are Systematic
- **Root Cause**: Mixed dependency specifications across workspace
- **Pattern**: Same dependency appears as `version="x"` in some crates, `{workspace=true}` in others
- **Solution**: Normalize ALL dependencies to workspace format

### 2. Alias Handling Is Critical
- **Problem**: `alloc` → `rustc-std-workspace-alloc`, `core` → `rustc-std-workspace-core`
- **Solution**: Extract actual crate name from path and rename dependencies
- **Implementation**: Check `path.file_name()` vs alias name in config

### 3. Path Validation Prevents Silent Failures
- **Issue**: Crates moved from `submodules/rust/compiler/` to `crates/`
- **Impact**: 88 paths needed updating in our case
- **Solution**: Automated path validation with common location search

### 4. Workspace Dependencies Must Be Complete
- **Problem**: Generated Cargo.toml files reference workspace deps that don't exist
- **Solution**: Auto-populate workspace from analysis results
- **Benefit**: Eliminates "failed to load manifest for dependency" errors

### 5. Bootstrap Creates Missing Wrapped Crates
- **Process**: Analysis → Path fixes → Regen → Bootstrap → Repeat
- **Result**: Each cycle resolves more dependencies and creates more wrapped crates
- **Convergence**: Eventually all dependencies resolve and workspace compiles

## Success Metrics

### Before Process
- Multiple dependency conflicts (serde, const-random, alloc, core, typed-arena)
- 88 invalid paths in configuration
- Compilation failures due to missing workspace dependencies

### After Process  
- Systematic resolution of conflicts one by one
- Automatic path validation and correction
- Progressive creation of missing wrapped crates
- Clear error progression showing forward movement

## Code Changes Made

### 1. Enhanced `normalize_syn_dependencies()`
```rust
// Convert ALL dependencies to workspace = true
for (_, dep_value) in dep_table.iter_mut() {
    match dep_value {
        toml::Value::String(_) => {
            *dep_value = workspace_table();
        }
        toml::Value::Table(table) => {
            table.remove("path");
            table.insert("workspace", true);
        }
    }
}
```

### 2. Added Path Validation to `update_split_decls_config`
```rust
// Check if path exists, try common locations if not
let possible_paths = [
    format!("crates/{}", crate_name),
    format!("submodules/{}", crate_name),
    format!("tools/{}", crate_name),
];
```

### 3. Dependency Analysis in `regen_cargo_v2`
```rust
// Analyze all dependencies and add missing ones to workspace
for (dep_name, stats) in &analysis.dependency_types {
    if !all_deps.contains_key(dep_name) {
        all_deps.insert(dep_name, workspace_entry);
    }
}
```

## Next Steps

1. **Complete Current Cycle**: Continue bootstrap → regen until clean compilation
2. **Automate Process**: Create single command that runs full cycle
3. **Add Validation**: Ensure workspace compiles before considering complete
4. **Document Edge Cases**: Handle special dependencies that can't use workspace format

## Tools Integration

The process successfully unified:
- **Analysis Tool**: Identifies problems systematically
- **Path Validator**: Fixes configuration automatically  
- **Dependency Normalizer**: Resolves conflicts consistently
- **Bootstrap Generator**: Creates missing components
- **Iterative Workflow**: Converges to working solution

This approach transforms an intractable manual debugging process into a systematic, automated resolution workflow.
