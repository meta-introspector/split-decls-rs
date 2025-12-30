// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < L , R , Target > AsMut < [Target] > for Either < L , R > where L : AsMut < [Target] > , R : AsMut < [Target] > , { fn as_mut (& mut self) -> & mut [Target] { for_both ! (self , inner => inner . as_mut ()) } }
};
}
