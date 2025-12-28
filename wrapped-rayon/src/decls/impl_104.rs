macro_rules! deps {
    () => {
        Iter!();
        ParallelDrainRange!();
        Drain!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a , T : Send > ParallelDrainRange < usize > for & 'a mut VecDeque < T > { type Iter = Drain < 'a , T > ; type Item = T ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { Drain { orig_len : self . len () , range : simplify_range (range , self . len ()) , deque : self , } } }
    };
}

impl_104!();