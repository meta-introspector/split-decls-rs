// Generated macro for impl_949 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_949 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_949"}
// Dependencies: {}
# [cfg (all (feature = "std" , unix))] impl Read for std :: fs :: File { fn read_exact_at (& self , buf : & mut [u8] , offset : u64) -> Result < () , Error > { use std :: os :: unix :: fs :: FileExt ; FileExt :: read_exact_at (self , buf , offset) . map_err (Error :: io) } }
};
}
