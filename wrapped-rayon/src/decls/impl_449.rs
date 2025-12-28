macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        # [doc = " Extends a string with strings from a parallel iterator."] impl ParallelExtend < String > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = String > , { extend_reserved ! (self , par_iter , string_len) ; } }
    };
}

impl_449!();