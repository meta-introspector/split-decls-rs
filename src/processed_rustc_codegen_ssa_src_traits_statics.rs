/* FP:statics.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_statics_USE_0001
/* FP:statics.rs-0002 */ use crate :: rustc_abi :: Align ;
/* FP:statics.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_statics_USE_0002
/* FP:statics.rs-0004 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:statics.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_statics_USE_0003
/* FP:statics.rs-0006 */ use super :: BackendTypes ;
/* FP:statics.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_statics_TRAIT_0004
/* FP:statics.rs-0008 */ pub trait StaticCodegenMethods : BackendTypes { fn static_addr_of (& self , cv : Self :: Value , align : Align , kind : Option < & str >) -> Self :: Value ; fn codegen_static (& mut self , def_id : DefId) ; }
/* FP:statics.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_statics_TRAIT_0005
/* FP:statics.rs-0010 */ pub trait StaticBuilderMethods : BackendTypes { fn get_static (& mut self , def_id : DefId) -> Self :: Value ; }