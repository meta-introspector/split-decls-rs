// Generated macro for impl_295 (impl)
macro_rules! Depcrate_zioimpl_295 {
() => {
// Module: crate::zio
// Provides: {"impl_295"}
// Dependencies: {}
impl Ops for Decompress { type Error = DecompressError ; type Flush = FlushDecompress ; fn total_in (& self) -> u64 { self . total_in () } fn total_out (& self) -> u64 { self . total_out () } fn run (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushDecompress ,) -> Result < Status , DecompressError > { self . decompress (input , output , flush) } fn run_vec (& mut self , input : & [u8] , output : & mut Vec < u8 > , flush : FlushDecompress ,) -> Result < Status , DecompressError > { self . decompress_vec (input , output , flush) } }
};
}
