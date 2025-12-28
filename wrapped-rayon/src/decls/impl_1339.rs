macro_rules! deps {
    () => {
        Drain!();
        Iter!();
        ParallelDrainRange!();
    };
}

macro_rules! impl_1339 {
    () => {
        deps!();
        impl < 'a > ParallelDrainRange < usize > for & 'a mut String { type Iter = Drain < 'a > ; type Item = char ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { Drain { range : simplify_range (range , self . len ()) , string : self , } } }
    };
}

impl_1339!();