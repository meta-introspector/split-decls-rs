// Generated macro for impl_116 (impl)
macro_rules! Depcrate_decodeimpl_116 {
() => {
// Module: crate::decode
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'de , R : Read > ReadSlice < 'de > for ReadReader < R > { # [inline] fn read_slice < 'a > (& 'a mut self , len : usize) -> Result < Reference < 'de , 'a , [u8] > , io :: Error > { self . buf . clear () ; let read = self . rd . by_ref () . take (len as u64) . read_to_end (& mut self . buf) ? ; if read != len { return Err (io :: ErrorKind :: UnexpectedEof . into ()) ; } Ok (Reference :: Copied (& self . buf [..])) } }
};
}
