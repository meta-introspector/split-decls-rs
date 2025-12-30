// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < L , R , Target > AsRef < Target > for Either < L , R > where L : AsRef < Target > , R : AsRef < Target > , { fn as_ref (& self) -> & Target { for_both ! (self , inner => inner . as_ref ()) } }
};
}
