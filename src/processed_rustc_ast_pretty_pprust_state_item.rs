/* FP:item.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0001
/* FP:item.rs-0002 */ use ast :: StaticItem ;
/* FP:item.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0002
/* FP:item.rs-0004 */ use itertools :: { Itertools , Position } ;
/* FP:item.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0003
/* FP:item.rs-0006 */ use rustc_ast as ast ;
/* FP:item.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0004
/* FP:item.rs-0008 */ use crate :: rustc_complete :: ModKind ;
/* FP:item.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0005
/* FP:item.rs-0010 */ use crate :: rustc_complete :: Ident ;
/* FP:item.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0006
/* FP:item.rs-0012 */ use crate :: pp :: BoxMarker ;
/* FP:item.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0007
/* FP:item.rs-0014 */ use crate :: pp :: Breaks :: Inconsistent ;
/* FP:item.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0008
/* FP:item.rs-0016 */ use crate :: pprust :: state :: fixup :: FixupContext ;
/* FP:item.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_USE_0009
/* FP:item.rs-0018 */ use crate :: pprust :: state :: { AnnNode , INDENT_UNIT , PrintState , State } ;
/* FP:item.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_ENUM_0010
/* FP:item.rs-0020 */ enum DelegationKind < 'a > { Single , List (& 'a [(Ident , Option < Ident >)]) , Glob , }
/* FP:item.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_FN_0011
/* FP:item.rs-0022 */ fn visibility_qualified (vis : & ast :: Visibility , s : & str) -> String { format ! ("{}{}" , State :: to_string (| s | s . print_visibility (vis)) , s) }
/* FP:item.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_pretty_src_pprust_state_item_IMPL_0012