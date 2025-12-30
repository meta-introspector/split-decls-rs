// Generated macro for impl_252 (impl)
macro_rules! Depcrate_tagimpl_252 {
() => {
// Module: crate::tag
// Provides: {"impl_252"}
// Dependencies: {}
impl DerOrd for Tag { fn der_cmp (& self , other : & Self) -> Result < Ordering > { Ok ((self . class () . cmp (& other . class ())) . then_with (| | self . number () . cmp (& other . number ())) . then_with (| | self . is_constructed () . cmp (& other . is_constructed ()))) } }
};
}
