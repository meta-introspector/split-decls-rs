# Rustc Crate Topology

## Independent Crates (35 total)
These crates have no rustc dependencies and can be built first:

- **rustc_fs_util**
- **rustc_attr_parsing**
- **rustc_error_codes**
- **rustc_expand**
- **rustc_thread_pool**
- **rustc_ast_passes**
- **rustc_log**
- **rustc_infer**
- **rustc_arena**
- **rustc_sanitizers**
- **rustc_metadata**
- **rustc_macros**
- **rustc_type_ir_macros**
- **rustc_interface**
- **rustc**
- **rustc_baked_icu_data**
- **rustc_ast**
- **rustc_data_structures**
- **rustc_ast_pretty**
- **rustc_index_macros**
- **rustc_index**
- **rustc_pattern_analysis**
- **rustc_serialize**
- **rustc_fluent_macro**
- **rustc_parse_format**
- **rustc_query_system**
- **rustc_driver**
- **rustc_next_trait_solver**
- **rustc_session**
- **rustc_hir**
- **rustc_llvm**
- **rustc_lexer**
- **rustc_middle**
- **rustc_trait_selection**
- **rustc_graphviz**

## Topological Order
Build order from least to most dependent:

1. **rustc_middle** (0 dependencies)
2. **rustc_macros** (0 dependencies)
3. **rustc_data_structures** (0 dependencies)
4. **rustc_hashes** (1 dependencies)
5. **rustc_serialize** (0 dependencies)
6. **rustc_span** (4 dependencies)
7. **rustc_query_system** (0 dependencies)
8. **rustc_query_impl** (4 dependencies)
9. **rustc_fs_util** (0 dependencies)
10. **rustc_index** (0 dependencies)
11. **rustc_abi** (4 dependencies)
12. **rustc_mir_build** (1 dependencies)
13. **rustc_session** (0 dependencies)
14. **rustc_hir** (0 dependencies)
15. **rustc_metadata** (0 dependencies)
16. **rustc_codegen_ssa** (8 dependencies)
17. **rustc_transmute** (3 dependencies)
18. **rustc_traits** (1 dependencies)
19. **rustc_ast** (0 dependencies)
20. **rustc_ast_pretty** (0 dependencies)
21. **rustc_error_messages** (3 dependencies)
22. **rustc_hir_id** (3 dependencies)
23. **rustc_lint_defs** (6 dependencies)
24. **rustc_errors** (5 dependencies)
25. **rustc_parse** (5 dependencies)
26. **rustc_public_bridge** (3 dependencies)
27. **rustc_privacy** (7 dependencies)
28. **rustc_attr_parsing** (0 dependencies)
29. **rustc_error_codes** (0 dependencies)
30. **rustc_expand** (0 dependencies)
31. **rustc_thread_pool** (0 dependencies)
32. **rustc_ast_passes** (0 dependencies)
33. **rustc_log** (0 dependencies)
34. **rustc_infer** (0 dependencies)
35. **rustc_driver** (0 dependencies)
36. **rustc_incremental** (1 dependencies)
37. **rustc_symbol_mangling** (3 dependencies)
38. **rustc_codegen_cranelift** (16 dependencies)
39. **rustc_arena** (0 dependencies)
40. **rustc_sanitizers** (0 dependencies)
41. **rustc_trait_selection** (0 dependencies)
42. **rustc_hir_analysis** (6 dependencies)
43. **rustc_type_ir_macros** (0 dependencies)
44. **rustc_interface** (0 dependencies)
45. **rustc_hir_pretty** (5 dependencies)
46. **rustc** (0 dependencies)
47. **rustc_baked_icu_data** (0 dependencies)
48. **rustc_ast_lowering** (10 dependencies)
49. **rustc_const_eval** (1 dependencies)
50. **rustc_builtin_macros** (2 dependencies)
51. **rustc_index_macros** (0 dependencies)
52. **rustc_codegen_llvm** (8 dependencies)
53. **rustc_type_ir** (1 dependencies)
54. **rustc_feature** (1 dependencies)
55. **rustc_mir_dataflow** (1 dependencies)
56. **rustc_pattern_analysis** (0 dependencies)
57. **rustc_fluent_macro** (0 dependencies)
58. **rustc_parse_format** (0 dependencies)
59. **rustc_public** (1 dependencies)
60. **rustc_mir_transform** (7 dependencies)
61. **rustc_passes** (1 dependencies)
62. **rustc_codegen_gcc** (19 dependencies)
63. **rustc_next_trait_solver** (0 dependencies)
64. **rustc_borrowck** (10 dependencies)
65. **rustc_lint** (2 dependencies)
66. **rustc_driver_impl** (14 dependencies)
67. **rustc_hir_typeck** (8 dependencies)
68. **rustc_monomorphize** (3 dependencies)
69. **rustc_llvm** (0 dependencies)
70. **rustc_lexer** (0 dependencies)
71. **rustc_ty_utils** (1 dependencies)
72. **rustc_resolve** (13 dependencies)
73. **rustc_ast_ir** (2 dependencies)
74. **rustc_graphviz** (0 dependencies)

## Dependency Details

### rustc_hashes
Depends on: rustc_stable_hash

### rustc_span
Depends on: rustc_macros, rustc_data_structures, rustc_hashes, rustc_serialize

### rustc_query_impl
Depends on: rustc_middle, rustc_span, rustc_query_system, rustc_data_structures

### rustc_abi
Depends on: rustc_data_structures, rustc_index, rustc_hashes, rustc_macros

### rustc_mir_build
Depends on: rustc_middle

### rustc_codegen_ssa
Depends on: rustc_middle, rustc_session, rustc_serialize, rustc_hir, rustc_span, rustc_data_structures, rustc_metadata, rustc_macros

### rustc_transmute
Depends on: rustc_middle, rustc_span, rustc_hir

### rustc_traits
Depends on: rustc_middle

### rustc_error_messages
Depends on: rustc_span, rustc_data_structures, rustc_macros

### rustc_hir_id
Depends on: rustc_macros, rustc_data_structures, rustc_span

### rustc_lint_defs
Depends on: rustc_macros, rustc_data_structures, rustc_error_messages, rustc_ast, rustc_span, rustc_hir_id

### rustc_errors
Depends on: rustc_span, rustc_data_structures, rustc_lint_defs, rustc_hashes, rustc_macros

### rustc_parse
Depends on: rustc_ast, rustc_span, rustc_session, rustc_ast_pretty, rustc_errors

### rustc_public_bridge
Depends on: rustc_data_structures, rustc_span, rustc_middle

### rustc_privacy
Depends on: rustc_errors, rustc_data_structures, rustc_middle, rustc_session, rustc_ast, rustc_span, rustc_hir

### rustc_incremental
Depends on: rustc_middle

### rustc_symbol_mangling
Depends on: rustc_session, rustc_hir, rustc_middle

### rustc_codegen_cranelift
Depends on: rustc_hir, rustc_errors, rustc_middle, rustc_codegen_ssa, rustc_session, rustc_fs_util, rustc_span, rustc_driver, rustc_abi, rustc_target, rustc_ast, rustc_data_structures, rustc_incremental, rustc_metadata, rustc_index, rustc_symbol_mangling

### rustc_hir_analysis
Depends on: rustc_span, rustc_middle, rustc_hir, rustc_abi, rustc_session, rustc_trait_selection

### rustc_hir_pretty
Depends on: rustc_ast_pretty, rustc_abi, rustc_hir, rustc_span, rustc_ast

### rustc_ast_lowering
Depends on: rustc_hir, rustc_macros, rustc_errors, rustc_middle, rustc_ast, rustc_index, rustc_session, rustc_attr_parsing, rustc_data_structures, rustc_span

### rustc_const_eval
Depends on: rustc_middle

### rustc_builtin_macros
Depends on: rustc_span, rustc_expand

### rustc_codegen_llvm
Depends on: rustc_middle, rustc_errors, rustc_codegen_ssa, rustc_data_structures, rustc_ast, rustc_metadata, rustc_span, rustc_session

### rustc_type_ir
Depends on: rustc_macros

### rustc_feature
Depends on: rustc_span

### rustc_mir_dataflow
Depends on: rustc_middle

### rustc_public
Depends on: rustc_public_bridge

### rustc_mir_transform
Depends on: rustc_index, rustc_hir, rustc_const_eval, rustc_data_structures, rustc_middle, rustc_mir_build, rustc_span

### rustc_passes
Depends on: rustc_middle

### rustc_codegen_gcc
Depends on: rustc_ast, rustc_index, rustc_target, rustc_errors, rustc_codegen_ssa, rustc_fs_util, rustc_symbol_mangling, rustc_fluent_macro, rustc_interface, rustc_span, rustc_abi, rustc_apfloat, rustc_hir, rustc_session, rustc_data_structures, rustc_middle, rustc_macros, rustc_type_ir, rustc_driver

### rustc_borrowck
Depends on: rustc_abi, rustc_index, rustc_middle, rustc_hir, rustc_errors, rustc_mir_dataflow, rustc_session, rustc_span, rustc_data_structures, rustc_infer

### rustc_lint
Depends on: rustc_hir, rustc_middle

### rustc_driver_impl
Depends on: rustc_feature, rustc_session, rustc_interface, rustc_data_structures, rustc_ast, rustc_index, rustc_metadata, rustc_parse, rustc_errors, rustc_lint, rustc_middle, rustc_span, rustc_target, rustc_codegen_ssa

### rustc_hir_typeck
Depends on: rustc_hir, rustc_hir_analysis, rustc_data_structures, rustc_infer, rustc_span, rustc_middle, rustc_errors, rustc_session

### rustc_monomorphize
Depends on: rustc_hir, rustc_middle, rustc_span

### rustc_ty_utils
Depends on: rustc_middle

### rustc_resolve
Depends on: rustc_arena, rustc_data_structures, rustc_feature, rustc_session, rustc_expand, rustc_hir, rustc_query_system, rustc_span, rustc_ast, rustc_metadata, rustc_errors, rustc_index, rustc_middle

### rustc_ast_ir
Depends on: rustc_macros, rustc_span

