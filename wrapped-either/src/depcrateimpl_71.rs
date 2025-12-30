// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < L , R , Target > AsMut < Target > for Either < L , R > where L : AsMut < Target > , R : AsMut < Target > , { fn as_mut (& mut self) -> & mut Target { for_both ! (self , inner => inner . as_mut ()) } }
};
}
