macro_rules! deps {
    () => {
        WeightedIndexIter!();
        SampleUniform!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < X > Debug for WeightedIndexIter < '_ , X > where X : SampleUniform + PartialOrd + Debug , X :: Sampler : Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WeightedIndexIter") . field ("weighted_index" , & self . weighted_index) . field ("index" , & self . index) . finish () } }
    };
}

impl_187!()