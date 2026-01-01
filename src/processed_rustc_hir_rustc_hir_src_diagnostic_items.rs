/* FP:diagnostic_items.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_USE_0001
/* FP:diagnostic_items.rs-0002 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:diagnostic_items.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_USE_0002
/* FP:diagnostic_items.rs-0004 */ use crate :: rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;
/* FP:diagnostic_items.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_USE_0003
/* FP:diagnostic_items.rs-0006 */ use crate :: rustc_complete :: Symbol ;
/* FP:diagnostic_items.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_USE_0004
/* FP:diagnostic_items.rs-0008 */ use crate :: rustc_complete :: def_id :: DefIdMap ;
/* FP:diagnostic_items.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_USE_0005
/* FP:diagnostic_items.rs-0010 */ use crate :: def_id :: DefId ;
/* FP:diagnostic_items.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_STRUCT_0006
/* FP:diagnostic_items.rs-0012 */ # [derive (Debug , Default)] pub struct DiagnosticItems { pub id_to_name : DefIdMap < Symbol > , pub name_to_id : FxIndexMap < Symbol , DefId > , }
/* FP:diagnostic_items.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_diagnostic_items_IMPL_0007
/* FP:diagnostic_items.rs-0014 */ impl < CTX : crate :: HashStableContext > HashStable < CTX > for DiagnosticItems { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . name_to_id . hash_stable (ctx , hasher) ; } }