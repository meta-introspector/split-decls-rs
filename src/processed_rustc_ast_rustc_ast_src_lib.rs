/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0007
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0009
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0010
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0011
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_MOD_0012
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_USE_0013
/* FP:lib.rs-0026 */ pub use self :: ast :: * ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_USE_0014
/* FP:lib.rs-0028 */ pub use self :: ast_traits :: { AstNodeWrapper , HasAttrs , HasNodeId , HasTokens } ;
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_lib_TRAIT_0015
/* FP:lib.rs-0030 */ # [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : crate :: rustc_span :: HashStableContext { }