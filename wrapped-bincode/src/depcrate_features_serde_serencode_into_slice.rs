// Generated macro for encode_into_slice (function)
macro_rules! Depcrate_features_serde_serencode_into_slice {
() => {
// Module: crate::features::serde::ser
// Provides: {"encode_into_slice"}
// Dependencies: {}
# [doc = " Encode the given value into the given slice. Returns the amount of bytes that have been written."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: ../config/index.html"] pub fn encode_into_slice < E , C > (val : E , dst : & mut [u8] , config : C) -> Result < usize , EncodeError > where E : Serialize , C : Config , { let mut encoder = crate :: enc :: EncoderImpl :: new (crate :: enc :: write :: SliceWriter :: new (dst) , config) ; let serializer = SerdeEncoder { enc : & mut encoder } ; val . serialize (serializer) ? ; Ok (encoder . into_writer () . bytes_written ()) }
};
}
