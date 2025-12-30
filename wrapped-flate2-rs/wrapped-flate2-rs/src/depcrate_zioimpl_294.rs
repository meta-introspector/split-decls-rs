// Generated macro for impl_294 (impl)
macro_rules! Depcrate_zioimpl_294 {
() => {
// Module: crate::zio
// Provides: {"impl_294"}
// Dependencies: {}
impl Ops for Compress { type Error = CompressError ; type Flush = FlushCompress ; fn total_in (& self) -> u64 { self . total_in () } fn total_out (& self) -> u64 { self . total_out () } fn run (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushCompress ,) -> Result < Status , CompressError > { self . compress (input , output , flush) } fn run_vec (& mut self , input : & [u8] , output : & mut Vec < u8 > , flush : FlushCompress ,) -> Result < Status , CompressError > { self . compress_vec (input , output , flush) } }
};
}
