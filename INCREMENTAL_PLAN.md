# Incremental Partition Testing Plan

## Goal
Test partition compilation incrementally to identify and fix problematic modules one by one.

## Current Status ✅
- Created `llm_macros` proc-macro crate for `llm_error_message` and `llm_context` attributes
- Created `partition1-crate` test environment
- Successfully compiled single module: `wrapped_futures_test_decls_module_not_found_stream`
- Created `partition_1_single.rs` with 8 modules commented out

## Next Steps
1. **Incremental Testing**: Uncomment modules one by one in `partition_1_single.rs`
2. **Error Identification**: For each failing module, identify specific compilation errors
3. **Fix Development**: Add necessary patches to `llm_macros` or `partition_module!` macro
4. **Validation**: Ensure each module compiles before adding the next

## Test Modules (in order)
- ✅ `wrapped_futures_test_decls_module_not_found_stream` - WORKING
- ⏳ `wrapped_mockall_derive_decls_concretize_args`
- ⏳ `wrapped_rocksdb_decls_module_not_found_snapshot`
- ⏳ `wrapped_futures_macro_decls_module_not_found_executor`
- ⏳ `wrapped_hir_decls_impl_for_TypeAlias`
- ⏳ `wrapped_hir_ty_decls_DeclContext`
- ⏳ `wrapped_rustc_builtin_macros_decls_module_not_found_cfg`
- ⏳ `wrapped_rustc_codegen_llvm_decls_module_not_found_attributes`
- ⏳ `wrapped_unicode_bom_decls_impl_for_From_u8_`

## Commands
- Test: `cd partition1-crate && cargo check`
- Check errors: `cat partition1-crate/build.log | grep error | sort | uniq -c | sort -rn | head`
- Edit test file: `partition_1_single.rs`
