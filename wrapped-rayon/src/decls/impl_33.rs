macro_rules! deps {
    () => {
        ParallelDrainFull!();
        Drain!();
        Iter!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , T : Ord + Send > ParallelDrainFull for & 'a mut BinaryHeap < T > { type Iter = Drain < 'a , T > ; type Item = T ; fn par_drain (self) -> Self :: Iter { Drain { heap : self } } }
    };
}

impl_33!();