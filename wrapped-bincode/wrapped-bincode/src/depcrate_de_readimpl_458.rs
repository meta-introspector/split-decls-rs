// Generated macro for impl_458 (impl)
macro_rules! Depcrate_de_readimpl_458 {
() => {
// Module: crate::de::read
// Provides: {"impl_458"}
// Dependencies: {}
impl < 'storage > BorrowReader < 'storage > for SliceReader < 'storage > { # [inline (always)] fn take_bytes (& mut self , length : usize) -> Result < & 'storage [u8] , DecodeError > { if length > self . slice . len () { return Err (DecodeError :: UnexpectedEnd { additional : length - self . slice . len () , }) ; } let (read_slice , remaining) = self . slice . split_at (length) ; self . slice = remaining ; Ok (read_slice) } }
};
}
