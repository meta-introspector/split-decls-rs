// Generated macro for encode_into_writer (function)
macro_rules! Depcrate_features_serde_serencode_into_writer {
() => {
// Module: crate::features::serde::ser
// Provides: {"encode_into_writer"}
// Dependencies: {}
# [doc = " Encode the given value into a custom [Writer]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: ../config/index.html"] pub fn encode_into_writer < E : Serialize , W : Writer , C : Config > (val : E , writer : W , config : C ,) -> Result < () , EncodeError > { let mut encoder = crate :: enc :: EncoderImpl :: < _ , C > :: new (writer , config) ; let serializer = SerdeEncoder { enc : & mut encoder } ; val . serialize (serializer) ? ; Ok (()) }
};
}
