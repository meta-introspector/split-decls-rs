macro_rules! deps {
    () => {
        ParallelDrainFull!();
        Drain!();
        Iter!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a , K : Send , V : Send , S > ParallelDrainFull for & 'a mut HashMap < K , V , S > { type Iter = Drain < 'a , K , V > ; type Item = (K , V) ; fn par_drain (self) -> Self :: Iter { let vec : Vec < _ > = self . drain () . collect () ; Drain { inner : vec . into_par_iter () , marker : PhantomData , } } }
    };
}

impl_68!()