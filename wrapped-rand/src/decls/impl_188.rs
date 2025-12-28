macro_rules! deps {
    () => {
        WeightedIndexIter!();
        SampleUniform!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < X > Clone for WeightedIndexIter < '_ , X > where X : SampleUniform + PartialOrd , { fn clone (& self) -> Self { WeightedIndexIter { weighted_index : self . weighted_index , index : self . index , } } }
    };
}

impl_188!()