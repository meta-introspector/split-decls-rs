=== RUSTC INCLUDE-BASED COMPILATION ERROR REPORT ===

## Summary Statistics
- **Total modules with errors**: 500
- **Total processed modules**: 1611
- **Success rate**: 68.96%

## Top 20 Modules with Most Errors

| Rank | Error Count | Module | Status |
|------|-------------|--------|--------|
| 1 | 89 | src/processed_rustc_macros_rustc_macros_src_symbols.rs | 🔴 NEEDS_FIX |
| 2 | 51 | src/processed_rustc_resolve_rustc_resolve_src_lib.rs | 🔴 NEEDS_FIX |
| 3 | 45 | src/processed_rustc_hir_typeck_rustc_hir_typeck_src_cast.rs | 🔴 NEEDS_FIX |
| 4 | 42 | src/processed_rustc_public_rustc_public_src_ty.rs | 🔴 NEEDS_FIX |
| 5 | 42 | src/processed_rustc_passes_rustc_passes_src_check_attr.rs | 🔴 NEEDS_FIX |
| 6 | 39 | src/processed_rustc_public_convert_stable_ty.rs | 🔴 NEEDS_FIX |
| 7 | 38 | src/processed_rustc_codegen_ssa_rustc_codegen_ssa_src_base.rs | 🔴 NEEDS_FIX |
| 8 | 33 | src/processed_rustc_hir_typeck_src_fn_ctxt_checks.rs | 🔴 NEEDS_FIX |
| 9 | 32 | src/processed_rustc_public_convert_stable_mir.rs | 🔴 NEEDS_FIX |
| 10 | 29 | src/processed_rustc_hir_analysis_src_hir_ty_lowering_mod.rs | 🔴 NEEDS_FIX |
| 11 | 29 | src/processed_rustc_codegen_ssa_src_back_write.rs | 🔴 NEEDS_FIX |
| 12 | 27 | src/processed_rustc_hir_analysis_src_check_wfcheck.rs | 🔴 NEEDS_FIX |
| 13 | 26 | src/processed_rustc_metadata_rustc_metadata_src_creader.rs | 🔴 NEEDS_FIX |
| 14 | 26 | src/processed_rustc_hir_analysis_src_check_mod.rs | 🔴 NEEDS_FIX |
| 15 | 26 | src/processed_rustc_expand_src_mbe_macro_rules.rs | 🔴 NEEDS_FIX |
| 16 | 26 | src/processed_rustc_expand_rustc_expand_src_base.rs | 🔴 NEEDS_FIX |
| 17 | 26 | src/processed_rustc_codegen_ssa_src_back_link.rs | 🔴 NEEDS_FIX |
| 18 | 25 | src/processed_rustc_resolve_rustc_resolve_src_late.rs | 🔴 NEEDS_FIX |
| 19 | 24 | src/processed_rustc_metadata_src_rmeta_mod.rs | 🔴 NEEDS_FIX |
| 20 | 24 | src/processed_rustc_metadata_src_rmeta_encoder.rs | 🔴 NEEDS_FIX |

## Error Categories Analysis

- **   1897 **: 1897 occurrences
- **rustc_complete**: 1282 occurrences
- **errors`**: 74 occurrences
- **ty`**: 37 occurrences
- **prelude`**: 35 occurrences
- **|     ^^^^^^^^^^^^^ no `errors` in the root**: 26 occurrences
- **traits`**: 21 occurrences
- **rustc_data_structures`**: 20 occurrences
- **inherent`**: 17 occurrences
- **rustc_trait_selection`**: 16 occurrences

## Crate-Level Error Distribution

- **src/processed_rustc_codegen**: 786 errors
- **src/processed_rustc_hir**: 649 errors
- **src/processed_rustc_passes**: 249 errors
- **src/processed_rustc_public**: 228 errors
- **src/processed_rustc_expand**: 219 errors
- **src/processed_rustc_metadata**: 200 errors
- **src/processed_rustc_query**: 175 errors
- **src/processed_rustc_macros**: 135 errors
- **src/processed_rustc_resolve**: 115 errors
- **src/processed_rustc_interface**: 114 errors
- **src/processed_rustc_data**: 112 errors
- **src/processed_rustc_type**: 101 errors
- **src/processed_rustc_errors**: 92 errors
- **src/processed_rustc_incremental**: 75 errors
- **src/processed_rustc_pattern**: 58 errors

## Processing Strategy

### Phase 1: High-Impact Modules (80+ errors)
- [ ] src/processed_rustc_macros_rustc_macros_src_symbols.rs (89 errors)

### Phase 2: Medium-Impact Modules (20-79 errors)
- [ ] src/processed_rustc_resolve_rustc_resolve_src_lib.rs (51 errors)
- [ ] src/processed_rustc_hir_typeck_rustc_hir_typeck_src_cast.rs (45 errors)
- [ ] src/processed_rustc_public_rustc_public_src_ty.rs (42 errors)
- [ ] src/processed_rustc_passes_rustc_passes_src_check_attr.rs (42 errors)
- [ ] src/processed_rustc_public_convert_stable_ty.rs (39 errors)
- [ ] src/processed_rustc_codegen_ssa_rustc_codegen_ssa_src_base.rs (38 errors)
- [ ] src/processed_rustc_hir_typeck_src_fn_ctxt_checks.rs (33 errors)
- [ ] src/processed_rustc_public_convert_stable_mir.rs (32 errors)
- [ ] src/processed_rustc_hir_analysis_src_hir_ty_lowering_mod.rs (29 errors)
- [ ] src/processed_rustc_codegen_ssa_src_back_write.rs (29 errors)
- [ ] src/processed_rustc_hir_analysis_src_check_wfcheck.rs (27 errors)
- [ ] src/processed_rustc_metadata_rustc_metadata_src_creader.rs (26 errors)
- [ ] src/processed_rustc_hir_analysis_src_check_mod.rs (26 errors)
- [ ] src/processed_rustc_expand_src_mbe_macro_rules.rs (26 errors)
- [ ] src/processed_rustc_expand_rustc_expand_src_base.rs (26 errors)
- [ ] src/processed_rustc_codegen_ssa_src_back_link.rs (26 errors)
- [ ] src/processed_rustc_resolve_rustc_resolve_src_late.rs (25 errors)
- [ ] src/processed_rustc_metadata_src_rmeta_mod.rs (24 errors)
- [ ] src/processed_rustc_metadata_src_rmeta_encoder.rs (24 errors)
- [ ] src/processed_rustc_metadata_src_rmeta_decoder.rs (24 errors)
- [ ] src/processed_rustc_interface_rustc_interface_src_util.rs (24 errors)
- [ ] src/processed_rustc_expand_rustc_expand_src_expand.rs (24 errors)
- [ ] src/processed_rustc_query_impl_rustc_query_impl_src_plumbing.rs (23 errors)
- [ ] src/processed_rustc_metadata_rmeta_decoder_cstore_impl.rs (23 errors)
- [ ] src/processed_rustc_interface_rustc_interface_src_passes.rs (23 errors)
- [ ] src/processed_rustc_interface_rustc_interface_src_interface.rs (23 errors)
- [ ] src/processed_rustc_hir_analysis_src_collect_resolve_bound_vars.rs (23 errors)
- [ ] src/processed_rustc_hir_analysis_src_check_check.rs (23 errors)
- [ ] src/processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib.rs (23 errors)
- [ ] src/processed_rustc_macros_rustc_macros_src_query.rs (21 errors)
- [ ] src/processed_rustc_hir_analysis_src_hir_ty_lowering_errors.rs (21 errors)
- [ ] src/processed_rustc_hir_analysis_rustc_hir_analysis_src_collect.rs (21 errors)
- [ ] src/processed_rustc_data_structures_src_obligation_forest_mod.rs (21 errors)
- [ ] src/processed_rustc_resolve_src_late_diagnostics.rs (20 errors)
- [ ] src/processed_rustc_hir_analysis_src_collect_type_of.rs (20 errors)
- [ ] src/processed_rustc_hir_analysis_src_check_compare_impl_item.rs (20 errors)
- [ ] src/processed_rustc_codegen_ssa_src_traits_builder.rs (20 errors)
- [ ] src/processed_rustc_codegen_ssa_src_mir_block.rs (20 errors)

### Phase 3: Low-Impact Modules (1-19 errors)
- 461 modules with 1-19 errors each

## Key Insights

1. **Current Success Rate**: 68.96% (1,111 out of 1,611 modules compile successfully)
2. **Error Concentration**: Top 20 modules account for 699 errors
3. **Most Problematic Crates**: rustc_codegen (786 errors), rustc_hir (649 errors)
4. **Primary Error Type**: rustc_complete module structure issues (1,282 occurrences)

## Next Steps

1. **Target Phase 1**: Fix the single 80+ error module first
2. **Focus on rustc_complete**: Address module structure issues
3. **Systematic approach**: Process modules in error-count order
4. **Track progress**: Update this report as modules are fixed
