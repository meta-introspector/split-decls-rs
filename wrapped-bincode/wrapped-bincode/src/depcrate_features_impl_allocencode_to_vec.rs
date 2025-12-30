// Generated macro for encode_to_vec (function)
macro_rules! Depcrate_features_impl_allocencode_to_vec {
() => {
// Module: crate::features::impl_alloc
// Provides: {"encode_to_vec"}
// Dependencies: {}
# [doc = " Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information."] # [doc = ""] # [doc = " [config]: config/index.html"] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn encode_to_vec < E : enc :: Encode , C : Config > (val : E , config : C) -> Result < Vec < u8 > , EncodeError > { let size = { let mut size_writer = enc :: EncoderImpl :: < _ , C > :: new (SizeWriter :: default () , config) ; val . encode (& mut size_writer) ? ; size_writer . into_writer () . bytes_written } ; let writer = VecWriter :: with_capacity (size) ; let mut encoder = enc :: EncoderImpl :: < _ , C > :: new (writer , config) ; val . encode (& mut encoder) ? ; Ok (encoder . into_writer () . inner) }
};
}
