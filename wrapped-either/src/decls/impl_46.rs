macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [cfg (any (test , feature = "std"))] # [doc = " `Either<L, R>` implements `Write` if both `L` and `R` do."] # [doc = ""] # [doc = " Requires crate feature `\"std\"`"] impl < L , R > Write for Either < L , R > where L : Write , R : Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { for_both ! (self , inner => inner . write (buf)) } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { for_both ! (self , inner => inner . write_all (buf)) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { for_both ! (self , inner => inner . write_fmt (fmt)) } fn flush (& mut self) -> io :: Result < () > { for_both ! (self , inner => inner . flush ()) } }
    };
}

impl_46!();