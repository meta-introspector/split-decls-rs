/* FP:declare.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_declare_USE_0001
/* FP:declare.rs-0002 */ use crate :: rustc_complete :: attrs :: Linkage ;
/* FP:declare.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_declare_USE_0002
/* FP:declare.rs-0004 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:declare.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_declare_USE_0003
/* FP:declare.rs-0006 */ use crate :: rustc_complete :: mir :: mono :: Visibility ;
/* FP:declare.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_declare_USE_0004
/* FP:declare.rs-0008 */ use crate :: rustc_complete :: ty :: Instance ;
/* FP:declare.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_declare_TRAIT_0005
/* FP:declare.rs-0010 */ pub trait PreDefineCodegenMethods < 'tcx > { fn predefine_static (& mut self , def_id : DefId , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) ; fn predefine_fn (& mut self , instance : Instance < 'tcx > , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) ; }