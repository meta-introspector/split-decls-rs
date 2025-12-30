// Generated macro for impl_106 (impl)
macro_rules! Depcrate_features_impl_stdimpl_106 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_106"}
// Dependencies: {}
impl < R > Reader for std :: io :: BufReader < R > where R : std :: io :: Read , { fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { self . read_exact (bytes) . map_err (| inner | DecodeError :: Io { inner , additional : bytes . len () , }) } # [inline] fn peek_read (& mut self , n : usize) -> Option < & [u8] > { self . buffer () . get (.. n) } # [inline] fn consume (& mut self , n : usize) { < Self as std :: io :: BufRead > :: consume (self , n) ; } }
};
}
