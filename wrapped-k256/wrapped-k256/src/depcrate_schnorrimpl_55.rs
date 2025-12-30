// Generated macro for impl_55 (impl)
macro_rules! Depcrate_schnorrimpl_55 {
() => {
// Module: crate::schnorr
// Provides: {"impl_55"}
// Dependencies: {}
impl PartialEq for Signature { fn eq (& self , other : & Self) -> bool { (self . r == other . r) && (self . s . ct_eq (& other . s) . into ()) } }
};
}
