// Generated macro for impl_398 (impl)
macro_rules! Depcrate_arg_array_implimpl_398 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_398"}
// Dependencies: {}
impl < 'a , T : Arg + Append + Clone > Append for Cow < 'a , [T] > { fn append_by_ref (& self , i : & mut IterAppend) { (& * * self) . append_by_ref (i) } }
};
}
