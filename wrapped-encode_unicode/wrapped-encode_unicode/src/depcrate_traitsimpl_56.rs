// Generated macro for impl_56 (impl)
macro_rules! Depcrate_traitsimpl_56 {
() => {
// Module: crate::traits
// Provides: {"impl_56"}
// Dependencies: {}
impl < I : Iterator > IterExt for I { fn to_bytes (self) -> Utf8CharSplitter < Self :: Item , Self > where Self :: Item : Borrow < Utf8Char > { Utf8CharSplitter :: from (self) } fn to_units (self) -> Utf16CharSplitter < Self :: Item , Self > where Self :: Item : Borrow < Utf16Char > { Utf16CharSplitter :: from (self) } fn to_utf8chars (self) -> Utf8CharMerger < Self :: Item , Self > where Self :: Item : Borrow < u8 > { Utf8CharMerger :: from (self) } fn to_utf16chars (self) -> Utf16CharMerger < Self :: Item , Self > where Self :: Item : Borrow < u16 > { Utf16CharMerger :: from (self) } }
};
}
