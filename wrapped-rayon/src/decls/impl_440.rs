macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        # [doc = " Extends an OS-string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < Cow < 'a , OsStr > > for OsString { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = Cow < 'a , OsStr > > , { extend_reserved ! (self , par_iter , osstring_len) ; } }
    };
}

impl_440!()