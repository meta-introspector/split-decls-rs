// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T , const CAP : usize , B : Behavior > From < ArrayDeque < T , CAP , B > > for Vec < T > where Self : FromIterator < T > , { fn from (deque : ArrayDeque < T , CAP , B >) -> Self { deque . into_iter () . collect () } }
};
}
