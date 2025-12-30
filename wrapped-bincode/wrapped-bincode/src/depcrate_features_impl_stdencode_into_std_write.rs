// Generated macro for encode_into_std_write (function)
macro_rules! Depcrate_features_impl_stdencode_into_std_write {
() => {
// Module: crate::features::impl_std
// Provides: {"encode_into_std_write"}
// Dependencies: {}
# [doc = " Encode the given value into any type that implements `std::io::Write`, e.g. `std::fs::File`, with the given `Config`."] # [doc = " See the [config] module for more information."] # [doc = " Returns the amount of bytes written."] # [doc = ""] # [doc = " [config]: config/index.html"] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn encode_into_std_write < E : Encode , C : Config , W : std :: io :: Write > (val : E , dst : & mut W , config : C ,) -> Result < usize , EncodeError > { let writer = IoWriter :: new (dst) ; let mut encoder = EncoderImpl :: < _ , C > :: new (writer , config) ; val . encode (& mut encoder) ? ; Ok (encoder . into_writer () . bytes_written ()) }
};
}
