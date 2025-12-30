// Generated macro for impl_229 (impl)
macro_rules! Depcrate_stream_readimpl_229 {
() => {
// Module: crate::stream::read
// Provides: {"impl_229"}
// Dependencies: {}
impl < R : Read > StreamOnce for Stream < R > { type Token = u8 ; type Range = & 'static [u8] ; type Position = usize ; type Error = Error ; # [inline] fn uncons (& mut self) -> Result < u8 , StreamErrorFor < Self > > { match self . bytes . next () { Some (Ok (b)) => Ok (b) , Some (Err (err)) => Err (Error :: Io (err)) , None => Err (Error :: EndOfInput) , } } }
};
}
