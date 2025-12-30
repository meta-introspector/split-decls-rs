// Generated macro for take_from_bytes_crc32 (function)
macro_rules! Depcrate_detake_from_bytes_crc32 {
() => {
// Module: crate::de
// Provides: {"take_from_bytes_crc32"}
// Dependencies: {}
# [doc = " Conveniently deserialize a message of type `T` from a byte slice with a Crc. The unused portion (if any)"] # [doc = " of the byte slice is returned for further usage"] # [doc = ""] # [doc = " See the `de_flavors::crc` module for the complete set of functions."] # [cfg (feature = "use-crc")] # [cfg_attr (docsrs , doc (cfg (feature = "use-crc")))] # [inline] pub fn take_from_bytes_crc32 < 'a , T > (s : & 'a [u8] , digest : crc :: Digest < 'a , u32 > ,) -> Result < (T , & 'a [u8]) > where T : Deserialize < 'a > , { flavors :: crc :: take_from_bytes_u32 (s , digest) }
};
}
