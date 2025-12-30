// Generated macro for encode_to_vec (function)
macro_rules! Depcrate_features_serde_serencode_to_vec {
() => {
// Module: crate::features::serde::ser
// Provides: {"encode_to_vec"}
// Dependencies: {}
# [doc = " Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information."] # [doc = ""] # [doc = " [config]: ../config/index.html"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn encode_to_vec < E , C > (val : E , config : C) -> Result < Vec < u8 > , EncodeError > where E : Serialize , C : Config , { let mut encoder = crate :: enc :: EncoderImpl :: new (crate :: VecWriter :: default () , config) ; let serializer = SerdeEncoder { enc : & mut encoder } ; val . serialize (serializer) ? ; Ok (encoder . into_writer () . collect ()) }
};
}
