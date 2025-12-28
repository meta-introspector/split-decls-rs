macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        # [doc = " Extends a string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < & 'a str > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a str > , { extend_reserved ! (self , par_iter , string_len) ; } }
    };
}

impl_448!();