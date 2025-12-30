// Generated macro for impl_202 (impl)
macro_rules! Depcrate_read_cfiimpl_202 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_202"}
// Dependencies: {}
impl < R : Reader > From < R > for DebugFrame < R > { fn from (section : R) -> Self { DebugFrame { section , address_size : mem :: size_of :: < usize > () as u8 , vendor : Vendor :: Default , } } }
};
}
