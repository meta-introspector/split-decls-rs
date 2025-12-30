// Generated macro for impl_16 (impl)
macro_rules! Depcrate_bufimpl_16 {
() => {
// Module: crate::buf
// Provides: {"impl_16"}
// Dependencies: {}
impl BufferFormat { # [doc = " Returns an error if the buffer format is not enabled."] pub fn check_available (& self) -> Result < () , DataError > { match self { # [cfg (feature = "deserialize_json")] BufferFormat :: Json => Ok (()) , # [cfg (not (feature = "deserialize_json"))] BufferFormat :: Json => Err (DataErrorKind :: Deserialize . with_str_context ("deserializing `BufferFormat::Json` requires the `deserialize_json` Cargo feature")) , # [cfg (feature = "deserialize_bincode_1")] BufferFormat :: Bincode1 => Ok (()) , # [cfg (not (feature = "deserialize_bincode_1"))] BufferFormat :: Bincode1 => Err (DataErrorKind :: Deserialize . with_str_context ("deserializing `BufferFormat::Bincode1` requires the `deserialize_bincode_1` Cargo feature")) , # [cfg (feature = "deserialize_postcard_1")] BufferFormat :: Postcard1 => Ok (()) , # [cfg (not (feature = "deserialize_postcard_1"))] BufferFormat :: Postcard1 => Err (DataErrorKind :: Deserialize . with_str_context ("deserializing `BufferFormat::Postcard1` requires the `deserialize_postcard_1` Cargo feature")) , } } }
};
}
