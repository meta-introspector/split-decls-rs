// Generated macro for Section (struct)
macro_rules! Depcrate_read_anySection {
() => {
// Module: crate::read::any
// Provides: {"Section"}
// Dependencies: {}
# [doc = " A section in a [`File`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] pub struct Section < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SectionInternal < 'data , 'file , R > , }
};
}
