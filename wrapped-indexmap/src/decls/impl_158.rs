macro_rules! deps {
    () => {
        IndexSet!();
        ParDrain!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'a , T , S > ParallelDrainRange < usize > for & 'a mut IndexSet < T , S > where T : Send , { type Item = T ; type Iter = ParDrain < 'a , T > ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { ParDrain { entries : self . map . core . par_drain (range) , } } }
    };
}

impl_158!()