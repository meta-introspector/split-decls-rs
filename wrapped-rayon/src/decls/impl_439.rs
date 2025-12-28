macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        # [doc = " Extends an OS-string with strings from a parallel iterator."] impl ParallelExtend < OsString > for OsString { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = OsString > , { extend_reserved ! (self , par_iter , osstring_len) ; } }
    };
}

impl_439!();