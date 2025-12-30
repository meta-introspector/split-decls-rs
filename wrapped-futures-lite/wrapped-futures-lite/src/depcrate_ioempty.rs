// Generated macro for empty (function)
macro_rules! Depcrate_ioempty {
() => {
// Module: crate::io
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Creates an empty reader."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{self, AsyncReadExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut reader = io::empty();"] # [doc = ""] # [doc = " let mut contents = Vec::new();"] # [doc = " reader.read_to_end(&mut contents).await?;"] # [doc = " assert!(contents.is_empty());"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn empty () -> Empty { Empty { _private : () } }
};
}
