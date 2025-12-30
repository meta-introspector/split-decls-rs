// Generated macro for impl_1211 (impl)
macro_rules! Depcrate_tagimpl_1211 {
() => {
// Module: crate::tag
// Provides: {"impl_1211"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for Tag < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Tag") ; if let Some (name) = self . name () { ds . field ("name" , & name) ; } ds . field ("id" , & self . id ()) ; ds . finish () } }
};
}
