macro_rules! deps {
    () => {
        ZeroWeightScale!();
        WeightScale!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < K , V > WeightScale < K , V > for ZeroWeightScale { # [inline] fn weight (& self , _ : & K , _ : & V) -> usize { 0 } }
    };
}

impl_21!();