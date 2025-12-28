macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        # [doc = " Extends a string with copied characters from a parallel iterator."] impl < 'a > ParallelExtend < & 'a char > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a char > , { self . par_extend (par_iter . into_par_iter () . copied ()) } }
    };
}

impl_442!();