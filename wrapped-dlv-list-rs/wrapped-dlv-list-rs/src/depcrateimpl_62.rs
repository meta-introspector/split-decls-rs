// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > Drain < '_ , T > { # [doc = " Creates an iterator that yields immutable references to entries in the list."] # [must_use] pub fn iter (& self) -> Iter < '_ , T > { Iter { entries : & self . list . entries , head : self . head , remaining : self . remaining , tail : self . tail , } } }
};
}
