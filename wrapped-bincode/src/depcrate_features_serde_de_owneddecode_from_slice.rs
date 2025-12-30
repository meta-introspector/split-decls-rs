// Generated macro for decode_from_slice (function)
macro_rules! Depcrate_features_serde_de_owneddecode_from_slice {
() => {
// Module: crate::features::serde::de_owned
// Provides: {"decode_from_slice"}
// Dependencies: {}
# [doc = " Attempt to decode a given type `D` from the given slice. Returns the decoded output and the amount of bytes read."] # [doc = ""] # [doc = " Note that this does not work with borrowed types like `&str` or `&[u8]`. For that use [borrow_decode_from_slice]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [borrow_decode_from_slice]: fn.borrow_decode_from_slice.html"] # [doc = " [config]: ../config/index.html"] pub fn decode_from_slice < D , C > (slice : & [u8] , config : C) -> Result < (D , usize) , DecodeError > where D : DeserializeOwned , C : Config , { borrow_decode_from_slice (slice , config) }
};
}
