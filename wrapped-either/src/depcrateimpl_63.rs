// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (any (test , feature = "std"))] # [doc = " `Either<L, R>` implements `Write` if both `L` and `R` do."] # [doc = ""] # [doc = " Requires crate feature `\"std\"`"] impl < L , R > Write for Either < L , R > where L : Write , R : Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { for_both ! (self , inner => inner . write (buf)) } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { for_both ! (self , inner => inner . write_all (buf)) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { for_both ! (self , inner => inner . write_fmt (fmt)) } fn flush (& mut self) -> io :: Result < () > { for_both ! (self , inner => inner . flush ()) } }
};
}
