// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < T , const CAP : usize , B : Behavior > Iterator for IntoIter < T , CAP , B > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . inner . pop_front () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . inner . len () ; (len , Some (len)) } }
};
}
