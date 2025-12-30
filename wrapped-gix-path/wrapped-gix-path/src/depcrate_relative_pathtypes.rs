// Generated macro for types (module)
macro_rules! Depcrate_relative_pathtypes {
() => {
// Module: crate::relative_path
// Provides: {"types"}
// Dependencies: {}
pub (super) mod types { use bstr :: { BStr , ByteSlice } ; # [doc = " A wrapper for `BStr`. It is used to enforce the following constraints:"] # [doc = ""] # [doc = " - The path separator always is `/`, independent of the platform."] # [doc = " - Only normal components are allowed."] # [doc = " - It is always represented as a bunch of bytes."] # [derive ()] pub struct RelativePath { inner : BStr , } impl AsRef < [u8] > for RelativePath { # [inline] fn as_ref (& self) -> & [u8] { self . inner . as_bytes () } } }
};
}
