macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
        FromParallelIterator!();
        Collector!();
    };
}

macro_rules! impl_966 {
    () => {
        deps!();
        impl < T , FromT > ParallelExtend < T > for Collector < FromT > where T : Send , FromT : Send + FromParallelIterator < T > , { fn par_extend < I > (& mut self , pi : I) where I : IntoParallelIterator < Item = T > , { debug_assert ! (self . result . is_none ()) ; self . result = Some (pi . into_par_iter () . collect ()) ; } }
    };
}

impl_966!();