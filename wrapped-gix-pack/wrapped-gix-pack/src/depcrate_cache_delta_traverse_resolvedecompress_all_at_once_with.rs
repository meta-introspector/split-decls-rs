// Generated macro for decompress_all_at_once_with (function)
macro_rules! Depcrate_cache_delta_traverse_resolvedecompress_all_at_once_with {
() => {
// Module: crate::cache::delta::traverse::resolve
// Provides: {"decompress_all_at_once_with"}
// Dependencies: {}
fn decompress_all_at_once_with (inflate : & mut zlib :: Inflate , b : & [u8] , decompressed_len : usize , out : & mut Vec < u8 > ,) -> Result < () , Error > { out . resize (decompressed_len , 0) ; inflate . reset () ; inflate . once (b , out) . map_err (| err | Error :: ZlibInflate { source : err , message : "Failed to decompress entry" , }) ? ; Ok (()) }
};
}
