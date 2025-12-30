// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl < L , R > DerefMut for Either < L , R > where L : DerefMut , R : DerefMut < Target = L :: Target > , { fn deref_mut (& mut self) -> & mut Self :: Target { for_both ! (self , inner => & mut * inner) } }
};
}
