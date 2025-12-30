// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > IterMut < '_ , T > { # [doc = " Creates an iterator that yields immutable references to entries in the list."] # [must_use] pub fn iter (& self) -> Iter < '_ , T > { Iter { entries : unsafe { & * self . entries } , head : self . head , remaining : self . remaining , tail : self . tail , } } }
};
}
