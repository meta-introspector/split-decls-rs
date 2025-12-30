// Generated macro for encode_into_slice (function)
macro_rules! Depcrateencode_into_slice {
() => {
// Module: crate
// Provides: {"encode_into_slice"}
// Dependencies: {}
# [doc = " Encode the given value into the given slice. Returns the amount of bytes that have been written."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn encode_into_slice < E : enc :: Encode , C : Config > (val : E , dst : & mut [u8] , config : C ,) -> Result < usize , error :: EncodeError > { let writer = enc :: write :: SliceWriter :: new (dst) ; let mut encoder = enc :: EncoderImpl :: < _ , C > :: new (writer , config) ; val . encode (& mut encoder) ? ; Ok (encoder . into_writer () . bytes_written ()) }
};
}
