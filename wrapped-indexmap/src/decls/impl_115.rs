macro_rules! deps {
    () => {
        ParDrain!();
        IndexMap!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < 'a , K , V , S > ParallelDrainRange < usize > for & 'a mut IndexMap < K , V , S > where K : Send , V : Send , { type Item = (K , V) ; type Iter = ParDrain < 'a , K , V > ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { ParDrain { entries : self . core . par_drain (range) , } } }
    };
}

impl_115!()