// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a > std :: fmt :: Debug for Candidate < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("Candidate") . field ("path" , & self . path . as_bstr ()) . field ("basename" , & self . basename . as_bstr ()) . field ("ext" , & self . ext . as_bstr ()) . finish () } }
};
}
