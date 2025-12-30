// Generated macro for DecodeError (enum)
macro_rules! Depcrate_features_serdeDecodeError {
() => {
// Module: crate::features::serde
// Provides: {"DecodeError"}
// Dependencies: {}
# [doc = " A serde-specific error that occurred while decoding."] # [derive (Debug)] # [non_exhaustive] pub enum DecodeError { # [doc = " Bincode does not support serde's `any` decoding feature."] # [doc = ""] # [doc = " See the \"known issues\" list in the serde module for more information on this."] AnyNotSupported , # [doc = " Bincode does not support serde identifiers"] IdentifierNotSupported , # [doc = " Bincode does not support serde's `ignored_any`."] # [doc = ""] # [doc = " See the \"known issues\" list in the serde module for more information on this."] IgnoredAnyNotSupported , # [doc = " Serde tried decoding a borrowed value from an owned reader. Use `serde_decode_borrowed_from_*` instead"] CannotBorrowOwnedData , # [doc = " Could not allocate data like `String` and `Vec<u8>`"] # [cfg (not (feature = "alloc"))] CannotAllocate , # [doc = " Custom serde error but bincode is unable to allocate a string. Set a breakpoint where this is thrown for more information."] # [cfg (not (feature = "alloc"))] CustomError , }
};
}
