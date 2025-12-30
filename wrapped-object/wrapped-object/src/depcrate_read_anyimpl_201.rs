// Generated macro for impl_201 (impl)
macro_rules! Depcrate_read_anyimpl_201 {
() => {
// Module: crate::read::any
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > fmt :: Debug for Comdat < 'data , 'file , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("Comdat") ; s . field ("symbol" , & self . symbol ()) . field ("name" , & self . name () . unwrap_or ("<invalid>")) . field ("kind" , & self . kind ()) . finish () } }
};
}
