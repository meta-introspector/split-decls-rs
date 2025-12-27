# Missing Files Manifest

## Critical binaries not found in output2:
- src/bin/report_aliases.rs → [MISSING]
- src/bin/regen_cargo.rs → [MISSING] 
- src/bin/regen_cargo_v2.rs → [MISSING]
- src/main.rs → [MISSING]

## Found in output2:
- src/generate_new_cargo.rs → wrapped_split_decls_rs_decls_module_not_found_generate_new_cargotoml.rs
- src/workspace_manager.rs → wrapped_split_decls_rs_decls_module_not_found_workspace_manager.rs
- src/process_crate.rs → wrapped_split_decls_rs_decls_module_not_found_process_crate.rs

## Action: Bootstrap needs to process src/bin/ directory
