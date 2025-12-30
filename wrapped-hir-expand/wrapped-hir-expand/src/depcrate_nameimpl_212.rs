// Generated macro for impl_212 (impl)
macro_rules! Depcrate_nameimpl_212 {
() => {
// Module: crate::name
// Provides: {"impl_212"}
// Dependencies: {}
impl fmt :: Debug for Name { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Name") . field ("symbol" , & self . symbol . as_str ()) . field ("ctx" , & self . ctx) . finish () } }
};
}
