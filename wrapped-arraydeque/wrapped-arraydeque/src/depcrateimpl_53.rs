// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < T , const CAP : usize , const N : usize , B : Behavior > From < [T ; N] > for ArrayDeque < T , CAP , B > where Self : FromIterator < T > , { fn from (arr : [T ; N]) -> Self { arr . into_iter () . collect () } }
};
}
