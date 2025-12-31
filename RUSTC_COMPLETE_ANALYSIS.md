# Complete rustc::main::main Function Analysis

## Summary
- **Total Functions Called: 6,218**
- **Unique Functions: 1,123**
- **Analysis Date: 2025-12-31**
- **Analysis Time: ~4 minutes**
- **Output Size: 471KB, 4,057 lines**

## Top 20 Most Called Functions
1. **346x** `rustc_resolve::errors::LowercaseSelf` - Name resolution errors
2. **302x** `rustc_middle::syntax::use_rustc_ast___{_InlineAsmOptions_,_InlineAsmTemplatePiece_,_Mutability_}` - AST/syntax handling
3. **257x** `rustc_ty_utils::implied_bounds::use_rustc_span___Span` - Type system bounds
4. **169x** `rustc_hir::intravisit::None` - HIR visitor pattern
5. **168x** `rustc_codegen_cranelift::linkage::use_rustc_middle___mir___mono___{_MonoItem_,_Visibility_}` - Code generation
6. **144x** `alloc::mod::Vec` - Vector operations
7. **112x** `alloc::string::String` - String operations
8. **107x** `rustc_mir_build::mod::ThenElseArgs` - MIR building
9. **104x** `rustc_resolve::macros::use_crate___errors___{_self_,_AddAsNonDerive_,_CannotDetermineMacroResolution_,_CannotFindIdentInThisScope_,_MacroExpectedFound_,_RemoveSurroundingDerive_,_}` - Macro resolution
10. **101x** `rustc_codegen_llvm::cpp_like::use_crate___debuginfo___metadata___{_DINodeCreationResult_,_NO_GENERICS_,_NO_SCOPE_METADATA_,_SmallVec_,_UNKNOWN_LINE_NUMBER_,_build_field_di_node_,_create_member_type_,_file_metadata_,_file_metadata_from_def_id_,_size_and_align_of_,_type_di_node_,_unknown_file_metadata_,_visibility_di_flags_,_}` - LLVM code generation
11. **101x** `rustc_type_ir::elaborate::ElaborateSized` - Type elaboration
12. **99x** `rustc_ast_pretty::state::use_crate___pp___{_self_,_BoxMarker_,_Breaks_}` - AST pretty printing
13. **87x** `rustc_monomorphize::partitioning::use_rustc_middle___middle___exported_symbols___{_SymbolExportInfo_,_SymbolExportLevel_}` - Monomorphization
14. **84x** `std::v1::use_crate___ops___{_Drop_,_Fn_,_FnMut_,_FnOnce_}` - Core operations
15. **69x** `rustc_middle::query::macro_call_rustc_index::newtype_index` - Query system
16. **64x** `rustc_codegen_gcc::mini_core_hello_world::SomeTrait` - GCC backend
17. **63x** `rustc_public::body::BasicBlockIdx` - Control flow
18. **57x** `core::control_flow::ControlFlow` - Control flow primitives
19. **50x** `core::iterator::Iterator` - Iterator trait
20. **44x** `std::futex::use_crate___sync___atomic___Ordering___Relaxed` - Atomic operations

## Compilation Pipeline Components Identified
- **Parser**: `rustc_ast`, `rustc_ast_pretty`
- **HIR**: `rustc_hir`, `rustc_hir_typeck`
- **MIR**: `rustc_mir_build`, `rustc_mir_transform`
- **Type System**: `rustc_ty_utils`, `rustc_type_ir`
- **Resolution**: `rustc_resolve`
- **Monomorphization**: `rustc_monomorphize`
- **Code Generation**: `rustc_codegen_llvm`, `rustc_codegen_cranelift`, `rustc_codegen_gcc`
- **Core Libraries**: `alloc`, `core`, `std`

## Files
- **Full Analysis**: `rustc_complete_analysis.txt` (471KB)
- **Symbol Database**: `symbol_map.json` (149,833 symbols)
