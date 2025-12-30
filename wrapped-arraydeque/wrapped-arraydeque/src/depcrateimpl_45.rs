// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < T , const CAP : usize > Clone for ArrayDeque < T , CAP , Wrapping > where T : Clone , { fn clone (& self) -> Self { self . iter () . cloned () . collect () } }
};
}
