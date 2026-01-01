/* FP:abi.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_abi_USE_0001
/* FP:abi.rs-0002 */ use super :: BackendTypes ;
/* FP:abi.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_abi_TRAIT_0002
/* FP:abi.rs-0004 */ pub trait AbiBuilderMethods : BackendTypes { fn get_param (& mut self , index : usize) -> Self :: Value ; }