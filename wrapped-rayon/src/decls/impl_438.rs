macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        # [doc = " Extends an OS-string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < & 'a OsStr > for OsString { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a OsStr > , { extend_reserved ! (self , par_iter , osstring_len) ; } }
    };
}

impl_438!();