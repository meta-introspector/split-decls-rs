macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1371 {
    () => {
        deps!();
        # [doc = " `Either<L, R>` can be extended if both `L` and `R` are parallel extendable."] impl < L , R , T > ParallelExtend < T > for Either < L , R > where L : ParallelExtend < T > , R : ParallelExtend < T > , T : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { match self . as_mut () { Left (collection) => collection . par_extend (par_iter) , Right (collection) => collection . par_extend (par_iter) , } } }
    };
}

impl_1371!()