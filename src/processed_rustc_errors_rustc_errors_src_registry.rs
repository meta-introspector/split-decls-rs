/* FP:registry.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_registry_USE_0001
/* FP:registry.rs-0002 */ use crate :: rustc_data_structures :: fx :: FxHashMap ;
/* FP:registry.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_registry_USE_0002
/* FP:registry.rs-0004 */ use crate :: ErrCode ;
/* FP:registry.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_registry_STRUCT_0003
/* FP:registry.rs-0006 */ # [derive (Debug)] pub struct InvalidErrorCode ;
/* FP:registry.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_registry_STRUCT_0004
/* FP:registry.rs-0008 */ # [derive (Clone)] pub struct Registry { long_descriptions : FxHashMap < ErrCode , & 'static str > , }
/* FP:registry.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_registry_IMPL_0005
/* FP:registry.rs-0010 */ impl Registry { pub fn new (long_descriptions : & [(ErrCode , & 'static str)]) -> Registry { Registry { long_descriptions : long_descriptions . iter () . copied () . collect () } } # [doc = " Returns `InvalidErrorCode` if the code requested does not exist in the"] # [doc = " registry."] pub fn try_find_description (& self , code : ErrCode) -> Result < & 'static str , InvalidErrorCode > { self . long_descriptions . get (& code) . copied () . ok_or (InvalidErrorCode) } }