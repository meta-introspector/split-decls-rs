// Generated macro for impl_219 (impl)
macro_rules! Depcrate_read_cfiimpl_219 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_219"}
// Dependencies: {}
impl < R : Reader > From < R > for EhFrame < R > { fn from (section : R) -> Self { EhFrame { section , address_size : mem :: size_of :: < usize > () as u8 , vendor : Vendor :: Default , } } }
};
}
