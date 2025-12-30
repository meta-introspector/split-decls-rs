// Generated macro for impl_122 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_122 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (feature = "std")] impl Read for Utf8Iterator { # [doc = " Always returns Ok"] fn read (& mut self , buf : & mut [u8]) -> Result < usize , ioError > { for (i , dst) in buf . iter_mut () . enumerate () { match self . next () { Some (b) => * dst = b , None => return Ok (i) , } } Ok (buf . len ()) } }
};
}
