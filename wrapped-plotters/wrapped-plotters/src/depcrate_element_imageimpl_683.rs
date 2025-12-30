// Generated macro for impl_683 (impl)
macro_rules! Depcrate_element_imageimpl_683 {
() => {
// Module: crate::element::image
// Provides: {"impl_683"}
// Dependencies: {}
impl < 'a > Buffer < 'a > { fn to_mut (& mut self) -> & mut [u8] { let owned = match self { Buffer :: Owned (owned) => return & mut owned [..] , Buffer :: BorrowedMut (target) => return target , Buffer :: Borrowed (target) => { let mut value = vec ! [] ; value . extend_from_slice (target) ; value } } ; * self = Buffer :: Owned (owned) ; self . to_mut () } }
};
}
