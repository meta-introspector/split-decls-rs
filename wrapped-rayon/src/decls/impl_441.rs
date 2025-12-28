macro_rules! deps {
    () => {
        ListStringConsumer!();
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        # [doc = " Extends a string with characters from a parallel iterator."] impl ParallelExtend < char > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = char > , { let list = par_iter . into_par_iter () . drive_unindexed (ListStringConsumer) ; self . reserve (list . iter () . map (String :: len) . sum ()) ; self . extend (list) ; } }
    };
}

impl_441!();