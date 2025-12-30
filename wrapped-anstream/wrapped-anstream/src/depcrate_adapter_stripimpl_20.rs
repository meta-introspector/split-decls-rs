// Generated macro for impl_20 (impl)
macro_rules! Depcrate_adapter_stripimpl_20 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_20"}
// Dependencies: {}
impl < 's > StrippedBytes < 's > { # [doc = " See [`strip_bytes`]"] # [inline] pub fn new (bytes : & 's [u8]) -> Self { Self { bytes , state : State :: Ground , utf8parser : Default :: default () , } } # [doc = " Strip the next slice of bytes"] # [doc = ""] # [doc = " Used when the content is in several non-contiguous slices"] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " May panic if it is not exhausted / empty"] # [inline] pub fn extend (& mut self , bytes : & 's [u8]) { debug_assert ! (self . is_empty () , "current bytes must be processed to ensure we end at the right state") ; self . bytes = bytes ; } # [doc = " Report the bytes has been exhausted"] # [inline] pub fn is_empty (& self) -> bool { self . bytes . is_empty () } # [doc = " Create a [`Vec`] of the printable content"] # [inline] pub fn into_vec (self) -> Vec < u8 > { let mut stripped = Vec :: with_capacity (self . bytes . len ()) ; for printable in self { stripped . extend (printable) ; } stripped } }
};
}
