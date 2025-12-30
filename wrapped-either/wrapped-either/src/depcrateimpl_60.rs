// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (any (test , feature = "std"))] # [doc = " `Either<L, R>` implements `Read` if both `L` and `R` do."] # [doc = ""] # [doc = " Requires crate feature `\"std\"`"] impl < L , R > Read for Either < L , R > where L : Read , R : Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { for_both ! (self , inner => inner . read (buf)) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { for_both ! (self , inner => inner . read_exact (buf)) } fn read_to_end (& mut self , buf : & mut std :: vec :: Vec < u8 >) -> io :: Result < usize > { for_both ! (self , inner => inner . read_to_end (buf)) } fn read_to_string (& mut self , buf : & mut std :: string :: String) -> io :: Result < usize > { for_both ! (self , inner => inner . read_to_string (buf)) } }
};
}
