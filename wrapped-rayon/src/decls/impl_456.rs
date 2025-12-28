macro_rules! deps {
    () => {
        NoopConsumer!();
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        # [doc = " Collapses all unit items from a parallel iterator into one."] impl ParallelExtend < () > for () { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = () > , { par_iter . into_par_iter () . drive_unindexed (NoopConsumer) } }
    };
}

impl_456!()