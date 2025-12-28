macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ListVecConsumer!();
        ParallelExtend!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        # [doc = " Extends a vector with items from a parallel iterator."] impl < T > ParallelExtend < T > for Vec < T > where T : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { let par_iter = par_iter . into_par_iter () ; match par_iter . opt_len () { Some (len) => { super :: collect :: special_extend (par_iter , len , self) ; } None => { let list = par_iter . drive_unindexed (ListVecConsumer) ; self . reserve (list . iter () . map (Vec :: len) . sum ()) ; for mut other in list { self . append (& mut other) ; } } } } }
    };
}

impl_454!()