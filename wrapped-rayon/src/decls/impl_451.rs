macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        # [doc = " Extends a string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < Cow < 'a , str > > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = Cow < 'a , str > > , { extend_reserved ! (self , par_iter , string_len) ; } }
    };
}

impl_451!();