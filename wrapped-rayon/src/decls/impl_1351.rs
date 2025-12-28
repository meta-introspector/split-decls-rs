macro_rules! deps {
    () => {
        Iter!();
        Drain!();
        ParallelDrainRange!();
    };
}

macro_rules! impl_1351 {
    () => {
        deps!();
        impl < 'data , T : Send > ParallelDrainRange < usize > for & 'data mut Vec < T > { type Iter = Drain < 'data , T > ; type Item = T ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { Drain { orig_len : self . len () , range : simplify_range (range , self . len ()) , vec : self , } } }
    };
}

impl_1351!();