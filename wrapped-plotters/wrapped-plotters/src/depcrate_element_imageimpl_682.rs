// Generated macro for impl_682 (impl)
macro_rules! Depcrate_element_imageimpl_682 {
() => {
// Module: crate::element::image
// Provides: {"impl_682"}
// Dependencies: {}
impl AsRef < [u8] > for Buffer < '_ > { fn as_ref (& self) -> & [u8] { match self { Buffer :: Owned (owned) => owned . as_ref () , Buffer :: Borrowed (target) => target , Buffer :: BorrowedMut (target) => target , } } }
};
}
