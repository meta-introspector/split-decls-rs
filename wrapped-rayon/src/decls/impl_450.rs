macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        # [doc = " Extends a string with boxed strings from a parallel iterator."] impl ParallelExtend < Box < str > > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = Box < str > > , { extend_reserved ! (self , par_iter , string_len) ; } }
    };
}

impl_450!()