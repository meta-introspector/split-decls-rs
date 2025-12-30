// Generated macro for impl_125 (impl)
macro_rules! Depcrate_adaptorsimpl_125 {
() => {
// Module: crate::adaptors
// Provides: {"impl_125"}
// Dependencies: {}
impl < I > PutBack < I > where I : Iterator , { # [doc = " put back value `value` (builder method)"] pub fn with_value (mut self , value : I :: Item) -> Self { self . put_back (value) ; self } # [doc = " Split the `PutBack` into its parts."] # [inline] pub fn into_parts (self) -> (Option < I :: Item > , I) { let Self { top , iter } = self ; (top , iter) } # [doc = " Put back a single value to the front of the iterator."] # [doc = ""] # [doc = " If a value is already in the put back slot, it is returned."] # [inline] pub fn put_back (& mut self , x : I :: Item) -> Option < I :: Item > { self . top . replace (x) } }
};
}
