/* FP:frozen.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_frozen_STRUCT_0001
/* FP:frozen.rs-0002 */ # [doc = " An owned immutable value."] # [derive (Debug , Clone)] pub struct Frozen < T > (T) ;
/* FP:frozen.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_frozen_IMPL_0002
/* FP:frozen.rs-0004 */ impl < T > Frozen < T > { pub fn freeze (val : T) -> Self { Frozen (val) } }
/* FP:frozen.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_frozen_IMPL_0003
/* FP:frozen.rs-0006 */ impl < T > std :: ops :: Deref for Frozen < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }