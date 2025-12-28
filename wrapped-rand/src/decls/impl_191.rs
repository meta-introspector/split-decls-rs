macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        WeightedIndex!();
        SampleUniform!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < X > Distribution < usize > for WeightedIndex < X > where X : SampleUniform + PartialOrd , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> usize { let chosen_weight = self . weight_distribution . sample (rng) ; self . cumulative_weights . partition_point (| w | w <= & chosen_weight) } }
    };
}

impl_191!();