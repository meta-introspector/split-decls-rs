# Split-Decls-RS Configuration System Documentation

## Overview
The `split-decls-rs.toml` file contains the complete configuration for the dependency resolution and workspace generation system.

## Configuration Sections

### `[wrapping].crates`
Lists all crates that should be wrapped and included in the generated workspace.
- Contains 600+ crate names from the entire dependency tree
- Each crate gets a `wrapped-{name}` directory in output2

### `[crate_path_overrides]` 
Maps crate names to their actual filesystem paths.
- Most point to `submodules/{crate-name}` for local dependencies
- Some have special paths like `/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/{crate}`
- External crates marked with `"*"` (like `icu_normalizer_data`, `detone`, `atoi`)

### `[workspace_dependency_overrides]`
Provides version and feature overrides for specific workspace dependencies.
- Example: `syn` gets `features = ["full"]` and `version = "2.0"`
- Example: `once_cell` gets `version = "1.18.0"`

## Current Issue
The workspace Cargo.toml generation is not properly using this configuration:
1. It's not reading the `crate_path_overrides` to determine local vs external deps
2. It's not applying the `workspace_dependency_overrides` 
3. It's generating incorrect workspace dependencies

## Solution
The workspace generation should:
1. Read `split-decls-rs.toml` configuration
2. For each crate in `wrapping.crates`:
   - Check `crate_path_overrides` for the path
   - If path exists and is local, add as `{ path = "path" }`
   - If marked as `"*"`, add as external with version from overrides
3. Apply all `workspace_dependency_overrides`
4. Generate proper workspace members list

## Tools That Use This Config
- `update_split_decls_config.rs` - Updates the configuration from Cargo.lock
- `regen_cargo.rs` - Should use this for Cargo.toml generation
- `gen_workspace.rs` - Should use this instead of parsing Cargo.lock directly
