// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl < L , R > Deref for Either < L , R > where L : Deref , R : Deref < Target = L :: Target > , { type Target = L :: Target ; fn deref (& self) -> & Self :: Target { for_both ! (self , inner => &** inner) } }
};
}
