// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T , const CAP : usize , B : Behavior > From < Vec < T > > for ArrayDeque < T , CAP , B > where Self : FromIterator < T > , { fn from (vec : Vec < T >) -> Self { vec . into_iter () . collect () } }
};
}
