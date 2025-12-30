// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_revwalkimpl_1110 {
() => {
// Module: crate::revwalk
// Provides: {"impl_1110"}
// Dependencies: {}
impl < 'repo , 'cb , C : FnMut (Oid) -> bool > RevwalkWithHideCb < 'repo , 'cb , C > { # [doc = " Consumes the `RevwalkWithHideCb` and returns the contained `Revwalk`."] # [doc = ""] # [doc = " Note that this will reset the `Revwalk`."] pub fn into_inner (mut self) -> Result < Revwalk < 'repo > , Error > { self . revwalk . reset () ? ; Ok (self . revwalk) } }
};
}
