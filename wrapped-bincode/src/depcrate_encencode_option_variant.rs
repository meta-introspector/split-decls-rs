// Generated macro for encode_option_variant (function)
macro_rules! Depcrate_encencode_option_variant {
() => {
// Module: crate::enc
// Provides: {"encode_option_variant"}
// Dependencies: {}
# [doc = " Encode the variant of the given option. Will not encode the option itself."] # [inline] pub (crate) fn encode_option_variant < E : Encoder , T > (encoder : & mut E , value : & Option < T > ,) -> Result < () , EncodeError > { match value { None => 0u8 . encode (encoder) , Some (_) => 1u8 . encode (encoder) , } }
};
}
