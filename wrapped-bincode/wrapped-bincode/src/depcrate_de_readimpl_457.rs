// Generated macro for impl_457 (impl)
macro_rules! Depcrate_de_readimpl_457 {
() => {
// Module: crate::de::read
// Provides: {"impl_457"}
// Dependencies: {}
impl < 'storage > Reader for SliceReader < 'storage > { # [inline (always)] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { if bytes . len () > self . slice . len () { return Err (DecodeError :: UnexpectedEnd { additional : bytes . len () - self . slice . len () , }) ; } let (read_slice , remaining) = self . slice . split_at (bytes . len ()) ; bytes . copy_from_slice (read_slice) ; self . slice = remaining ; Ok (()) } # [inline] fn peek_read (& mut self , n : usize) -> Option < & 'storage [u8] > { self . slice . get (.. n) } # [inline] fn consume (& mut self , n : usize) { self . slice = self . slice . get (n ..) . unwrap_or_default () ; } }
};
}
