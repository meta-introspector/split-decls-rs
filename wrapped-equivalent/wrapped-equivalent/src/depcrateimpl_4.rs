// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < Q : ? Sized , K : ? Sized > Equivalent < K > for Q where Q : Eq , K : Borrow < Q > , { # [inline] fn equivalent (& self , key : & K) -> bool { PartialEq :: eq (self , key . borrow ()) } }
};
}
