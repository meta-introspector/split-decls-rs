// Generated macro for impl_121 (impl)
macro_rules! Depcrate_zlibimpl_121 {
() => {
// Module: crate::zlib
// Provides: {"impl_121"}
// Dependencies: {}
impl Inflate { # [doc = " Run the decompressor exactly once. Cannot be run multiple times"] pub fn once (& mut self , input : & [u8] , out : & mut [u8]) -> Result < (Status , usize , usize) , inflate :: Error > { let before_in = self . state . total_in () ; let before_out = self . state . total_out () ; let status = self . state . decompress (input , out , FlushDecompress :: None) ? ; Ok ((status , (self . state . total_in () - before_in) as usize , (self . state . total_out () - before_out) as usize ,)) } # [doc = " Ready this instance for decoding another data stream."] pub fn reset (& mut self) { self . state . reset () ; } }
};
}
