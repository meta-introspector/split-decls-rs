// Generated macro for impl_29 (impl)
macro_rules! Depcrate_astimpl_29 {
() => {
// Module: crate::ast
// Provides: {"impl_29"}
// Dependencies: {}
impl PartialEq < & [Symbol] > for Path { # [inline] fn eq (& self , names : & & [Symbol]) -> bool { self . segments . len () == names . len () && self . segments . iter () . zip (names . iter ()) . all (| (s1 , s2) | s1 == s2) } }
};
}
