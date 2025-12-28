macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
        UnEither!();
    };
}

macro_rules! impl_959 {
    () => {
        deps!();
        impl < L , R , A , B > ParallelExtend < Either < L , R > > for (A , B) where L : Send , R : Send , A : Send + ParallelExtend < L > , B : Send + ParallelExtend < R > , { fn par_extend < I > (& mut self , pi : I) where I : IntoParallelIterator < Item = Either < L , R > > , { execute_into (& mut self . 0 , & mut self . 1 , pi . into_par_iter () , UnEither) ; } }
    };
}

impl_959!();