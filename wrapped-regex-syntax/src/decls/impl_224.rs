macro_rules! deps {
    () => {
        Result!();
        Hir!();
        Formatter!();
        Printer!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        # [doc = " Print a display representation of this Hir."] # [doc = ""] # [doc = " The result of this is a valid regular expression pattern string."] # [doc = ""] # [doc = " This implementation uses constant stack space and heap space proportional"] # [doc = " to the size of the `Hir`."] impl core :: fmt :: Display for Hir { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { crate :: hir :: print :: Printer :: new () . print (self , f) } }
    };
}

impl_224!()