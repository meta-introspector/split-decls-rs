// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < T , const CAP : usize > Clone for ArrayDeque < T , CAP , Saturating > where T : Clone , { fn clone (& self) -> Self { self . iter () . cloned () . collect () } }
};
}
