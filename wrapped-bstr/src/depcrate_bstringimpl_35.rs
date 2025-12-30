// Generated macro for impl_35 (impl)
macro_rules! Depcrate_bstringimpl_35 {
() => {
// Module: crate::bstring
// Provides: {"impl_35"}
// Dependencies: {}
impl BString { # [doc = " Constructs a new `BString` from the given [`Vec`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::BString;"] # [doc = ""] # [doc = " let mut b = BString::new(Vec::with_capacity(10));"] # [doc = " ```"] # [doc = ""] # [doc = " This function is `const`:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::BString;"] # [doc = ""] # [doc = " const B: BString = BString::new(vec![]);"] # [doc = " ```"] # [inline] pub const fn new (bytes : Vec < u8 >) -> BString { BString { bytes } } # [inline] pub (crate) fn as_bytes (& self) -> & [u8] { & self . bytes } # [inline] pub (crate) fn as_bytes_mut (& mut self) -> & mut [u8] { & mut self . bytes } # [inline] pub (crate) fn as_bstr (& self) -> & BStr { BStr :: new (& self . bytes) } # [inline] pub (crate) fn as_mut_bstr (& mut self) -> & mut BStr { BStr :: new_mut (& mut self . bytes) } # [inline] pub (crate) fn as_vec (& self) -> & Vec < u8 > { & self . bytes } # [inline] pub (crate) fn as_vec_mut (& mut self) -> & mut Vec < u8 > { & mut self . bytes } # [inline] pub (crate) fn into_vec (self) -> Vec < u8 > { self . bytes } }
};
}
