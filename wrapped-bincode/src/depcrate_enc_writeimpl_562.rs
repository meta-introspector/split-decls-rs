// Generated macro for impl_562 (impl)
macro_rules! Depcrate_enc_writeimpl_562 {
() => {
// Module: crate::enc::write
// Provides: {"impl_562"}
// Dependencies: {}
impl Writer for SliceWriter < '_ > { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { if bytes . len () > self . slice . len () { return Err (EncodeError :: UnexpectedEnd) ; } let (a , b) = core :: mem :: take (& mut self . slice) . split_at_mut (bytes . len ()) ; a . copy_from_slice (bytes) ; self . slice = b ; Ok (()) } }
};
}
