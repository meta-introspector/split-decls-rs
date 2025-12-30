// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Clone > Clone for COption < T > { # [inline] fn clone (& self) -> Self { match self { COption :: Some (x) => COption :: Some (x . clone ()) , COption :: None => COption :: None , } } # [inline] fn clone_from (& mut self , source : & Self) { match (self , source) { (COption :: Some (to) , COption :: Some (from)) => to . clone_from (from) , (to , from) => to . clone_from (from) , } } }
};
}
