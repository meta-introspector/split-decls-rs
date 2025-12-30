// Generated macro for impl_406 (impl)
macro_rules! Depcrate_arg_array_implimpl_406 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_406"}
// Dependencies: {}
impl < 'a , K : 'a + DictKey + Append , V : 'a + Append + Arg , I : Iterator < Item = (K , V) > + Clone > Append for Dict < 'a , K , V , I > { fn append_by_ref (& self , i : & mut IterAppend) { let z = self . 0 . clone () ; i . append_container (Self :: ARG_TYPE , Some (& CString :: new (Self :: entry_sig ()) . unwrap ()) , | s | for (k , v) in z { s . append_container (ArgType :: DictEntry , None , | ss | { k . append_by_ref (ss) ; v . append_by_ref (ss) ; }) }) ; } }
};
}
