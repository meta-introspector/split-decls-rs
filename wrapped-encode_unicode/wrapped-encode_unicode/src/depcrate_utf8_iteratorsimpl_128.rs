// Generated macro for impl_128 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_128 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "std")] impl < U : Borrow < Utf8Char > , I : Iterator < Item = U > > Read for Utf8CharSplitter < U , I > { # [doc = " Always returns `Ok`"] fn read (& mut self , buf : & mut [u8]) -> Result < usize , ioError > { let mut i = 0 ; while self . prev != 0 && i < buf . len () { buf [i] = self . prev as u8 ; self . prev >>= 8 ; i += 1 ; } while i < buf . len () { let bytes = match self . inner . next () { Some (u8c) => u8c . borrow () . to_array () . 0 , None => break } ; buf [i] = bytes [0] ; i += 1 ; if bytes [1] != 0 { let len = bytes [0] . not () . leading_zeros () as usize ; let mut written = 1 ; while written < len { if i < buf . len () { buf [i] = bytes [written] ; i += 1 ; written += 1 ; } else { let bytes_as_u32 = u32 :: from_le_bytes (bytes) ; self . prev = bytes_as_u32 >> (8 * written) ; return Ok (i) ; } } } } Ok (i) } }
};
}
