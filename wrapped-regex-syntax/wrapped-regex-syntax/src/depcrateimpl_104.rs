// Generated macro for impl_104 (impl)
macro_rules! Depcrateimpl_104 {
() => {
// Module: crate
// Provides: {"impl_104"}
// Dependencies: {}
impl Repeater { # [doc = " Returns true if and only if this repetition can match the empty string."] fn matches_empty (& self) -> bool { use self :: Repeater :: * ; match * self { ZeroOrOne => true , ZeroOrMore => true , OneOrMore => false , Range { min , .. } => min == 0 , } } }
};
}
