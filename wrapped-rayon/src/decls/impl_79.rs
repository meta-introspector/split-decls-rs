macro_rules! deps {
    () => {
        ParallelDrainFull!();
        Drain!();
        Iter!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , T : Send , S > ParallelDrainFull for & 'a mut HashSet < T , S > { type Iter = Drain < 'a , T > ; type Item = T ; fn par_drain (self) -> Self :: Iter { let vec : Vec < _ > = self . drain () . collect () ; Drain { inner : vec . into_par_iter () , marker : PhantomData , } } }
    };
}

impl_79!();