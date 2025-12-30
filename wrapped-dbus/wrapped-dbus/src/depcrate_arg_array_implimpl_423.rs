// Generated macro for impl_423 (impl)
macro_rules! Depcrate_arg_array_implimpl_423 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_423"}
// Dependencies: {}
impl < 'a , T : 'a + Arg + Append , I : Iterator < Item = T > + Clone > Append for Array < 'a , T , I > { fn append_by_ref (& self , i : & mut IterAppend) { let z = self . 0 . clone () ; i . append_container (ArgType :: Array , Some (T :: signature () . as_cstr ()) , | s | for arg in z { arg . append_by_ref (s) }) ; } }
};
}
