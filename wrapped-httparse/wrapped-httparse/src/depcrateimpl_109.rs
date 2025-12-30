// Generated macro for impl_109 (impl)
macro_rules! Depcrateimpl_109 {
() => {
// Module: crate
// Provides: {"impl_109"}
// Dependencies: {}
impl < T > Status < T > { # [doc = " Convenience method to check if status is complete."] # [inline] pub fn is_complete (& self) -> bool { match * self { Status :: Complete (..) => true , Status :: Partial => false } } # [doc = " Convenience method to check if status is partial."] # [inline] pub fn is_partial (& self) -> bool { match * self { Status :: Complete (..) => false , Status :: Partial => true } } # [doc = " Convenience method to unwrap a Complete value. Panics if the status is"] # [doc = " `Partial`."] # [inline] pub fn unwrap (self) -> T { match self { Status :: Complete (t) => t , Status :: Partial => panic ! ("Tried to unwrap Status::Partial") } } }
};
}
