// Generated macro for decode_from_std_read (function)
macro_rules! Depcrate_features_impl_stddecode_from_std_read {
() => {
// Module: crate::features::impl_std
// Provides: {"decode_from_std_read"}
// Dependencies: {}
# [doc = " Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`."] # [doc = ""] # [doc = " See the [config] module for more information about config options."] # [doc = ""] # [doc = " [config]: config/index.html"] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn decode_from_std_read < D : Decode < () > , C : Config , R : std :: io :: Read > (src : & mut R , config : C ,) -> Result < D , DecodeError > { decode_from_std_read_with_context (src , config , ()) }
};
}
