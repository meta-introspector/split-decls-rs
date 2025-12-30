// Generated macro for impl_1771 (impl)
macro_rules! Depcrate_perfcnt_intel_descriptionimpl_1771 {
() => {
// Module: crate::perfcnt::intel::description
// Provides: {"impl_1771"}
// Dependencies: {}
impl fmt :: Debug for PebsType { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let name = match * self { PebsType :: Regular => "Regular" , PebsType :: PebsOrRegular => "PebsOrRegular" , PebsType :: PebsOnly => "PebsOnly" , } ; write ! (f , "PebsType::{}" , name) } }
};
}
