// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl < T > IntoIter < T > { # [doc = " Creates an iterator that yields immutable references to entries in the list."] # [must_use] pub fn iter (& self) -> Iter < '_ , T > { Iter { entries : & self . list . entries , head : self . head , remaining : self . remaining , tail : self . tail , } } }
};
}
