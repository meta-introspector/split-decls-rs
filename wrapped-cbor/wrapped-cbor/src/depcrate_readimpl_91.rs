// Generated macro for impl_91 (impl)
macro_rules! Depcrate_readimpl_91 {
() => {
// Module: crate::read
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R > IoRead < R > where R : io :: Read , { # [doc = " Creates a new CBOR input source to read from a std::io input stream."] pub fn new (reader : R) -> IoRead < R > { IoRead { reader : OffsetReader { reader , offset : 0 } , scratch : vec ! [] , ch : None , } } # [inline] fn next_inner (& mut self) -> Result < Option < u8 > > { let mut buf = [0 ; 1] ; loop { match self . reader . read (& mut buf) { Ok (0) => return Ok (None) , Ok (_) => return Ok (Some (buf [0])) , Err (ref e) if e . kind () == io :: ErrorKind :: Interrupted => { } Err (e) => return Err (Error :: io (e)) , } } } }
};
}
