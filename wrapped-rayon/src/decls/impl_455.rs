macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        # [doc = " Extends a vector with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for Vec < T > where T : 'a + Copy + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { self . par_extend (par_iter . into_par_iter () . copied ()) } }
    };
}

impl_455!();