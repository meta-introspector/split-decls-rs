// Generated macro for impl_363 (impl)
macro_rules! Depcrate_arg_variantstruct_implimpl_363 {
() => {
// Module: crate::arg::variantstruct_impl
// Provides: {"impl_363"}
// Dependencies: {}
impl < T : Arg + Append > Append for Variant < T > { fn append_by_ref (& self , i : & mut IterAppend) { let z = & self . 0 ; i . append_container (ArgType :: Variant , Some (T :: signature () . as_cstr ()) , | s | z . append_by_ref (s)) ; } }
};
}
