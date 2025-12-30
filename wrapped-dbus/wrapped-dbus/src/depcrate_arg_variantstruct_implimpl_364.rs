// Generated macro for impl_364 (impl)
macro_rules! Depcrate_arg_variantstruct_implimpl_364 {
() => {
// Module: crate::arg::variantstruct_impl
// Provides: {"impl_364"}
// Dependencies: {}
impl Append for Variant < Box < dyn RefArg > > { fn append_by_ref (& self , i : & mut IterAppend) { let z = & self . 0 ; i . append_container (ArgType :: Variant , Some (z . signature () . as_cstr ()) , | s | z . append (s)) ; } }
};
}
