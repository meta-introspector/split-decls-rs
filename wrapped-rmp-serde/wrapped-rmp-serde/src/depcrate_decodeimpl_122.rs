// Generated macro for impl_122 (impl)
macro_rules! Depcrate_decodeimpl_122 {
() => {
// Module: crate::decode
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'de , T : AsRef < [u8] > + ? Sized > ReadSlice < 'de > for ReadRefReader < 'de , T > { # [inline] fn read_slice < 'a > (& 'a mut self , len : usize) -> Result < Reference < 'de , 'a , [u8] > , io :: Error > { if len > self . buf . len () { return Err (ErrorKind :: UnexpectedEof . into ()) ; } let (a , b) = self . buf . split_at (len) ; self . buf = b ; Ok (Reference :: Borrowed (a)) } }
};
}
