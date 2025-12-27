# Bootstrap Generation Manifest

## Purpose
Track which source files generate which output files and ensure critical binaries are included in bootstrap.

## Missing Binaries to Generate

### Critical Files Not Found in output2:
```toml
[missing_binaries]
report_aliases = "src/bin/report_aliases.rs"
regen_cargo = "src/bin/regen_cargo.rs" 
regen_cargo_v2 = "src/bin/regen_cargo_v2.rs"
audit_deps = "src/bin/audit_deps.rs"
wrap_single_crate = "src/bin/wrap_single_crate.rs"

[missing_core_files]
main = "src/main.rs"
patch_config = "src/patch_config.rs"
```

## File Mapping Manifest

### Source → Output Mapping:
```toml
[file_mappings]

# Core cargo handling
"src/generate_new_cargo.rs" = "output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_module_not_found_generate_new_cargotoml.rs"
"src/workspace_manager.rs" = "output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_module_not_found_workspace_manager.rs"
"src/process_crate.rs" = "output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_module_not_found_process_crate.rs"

# Missing binaries (need generation)
"src/bin/report_aliases.rs" = "output2/wrapped-split-decls-rs/src/decls/[MISSING]"
"src/bin/regen_cargo.rs" = "output2/wrapped-split-decls-rs/src/decls/[MISSING]"
"src/bin/regen_cargo_v2.rs" = "output2/wrapped-split-decls-rs/src/decls/[MISSING]"
"src/main.rs" = "output2/wrapped-split-decls-rs/src/decls/[MISSING]"
```

## Influence Tracking

### Dependencies per Output:
```toml
[influences]

# generate_new_cargotoml.rs influences:
"wrapped_split_decls_rs_decls_module_not_found_generate_new_cargotoml.rs" = [
    "src/generate_new_cargo.rs",           # Primary source
    "cargo-toml-generator-types",          # CargoToml struct
    "src/patch_config.rs",                 # SplitDeclsConfig
    "split-decls-rs.toml"                  # Configuration
]

# workspace_manager.rs influences:
"wrapped_split_decls_rs_decls_module_not_found_workspace_manager.rs" = [
    "src/workspace_manager.rs",            # Primary source
    "src/paths.rs",                        # Path handling
    "split-decls-rs.toml"                  # Workspace config
]

# process_crate.rs influences:
"wrapped_split_decls_rs_decls_module_not_found_process_crate.rs" = [
    "src/process_crate.rs",                # Primary source
    "src/generate_new_cargo.rs",           # Cargo generation
    "src/backup_original_cargo.rs",       # File backup
    "src/generate_new_lib_rs.rs"          # Lib generation
]
```

## Bootstrap Fix Requirements

### 1. Update split-decls-rs.toml:
```toml
[wrapping]
crates = [
    # ... existing crates ...
    "split-decls-rs",  # Ensure self-wrapping includes binaries
]

[binary_inclusion]
# Force inclusion of critical binaries
include_binaries = [
    "report_aliases",
    "regen_cargo", 
    "regen_cargo_v2",
    "audit_deps",
    "wrap_single_crate"
]
```

### 2. Bootstrap Generation Check:
```bash
# Verify these files exist after bootstrap:
output2/wrapped-split-decls-rs/src/decls/*report_aliases*
output2/wrapped-split-decls-rs/src/decls/*regen_cargo*
output2/wrapped-split-decls-rs/src/decls/*audit_deps*
output2/wrapped-split-decls-rs/src/decls/*main*
```

## Action Items

1. **Update bootstrap** to ensure `src/bin/*` files are processed
2. **Add binary detection** in main.rs to verify all critical tools are wrapped
3. **Create validation** that checks for missing binaries post-bootstrap
4. **Generate manifest** automatically during bootstrap process

## Expected Output Structure:
```
output2/wrapped-split-decls-rs/src/decls/
├── wrapped_split_decls_rs_decls_report_aliases.rs
├── wrapped_split_decls_rs_decls_regen_cargo.rs  
├── wrapped_split_decls_rs_decls_regen_cargo_v2.rs
├── wrapped_split_decls_rs_decls_audit_deps.rs
├── wrapped_split_decls_rs_decls_main.rs
└── wrapped_split_decls_rs_decls_patch_config.rs
```
