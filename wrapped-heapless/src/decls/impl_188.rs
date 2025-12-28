macro_rules! deps {
    () => {
        LinearMapInner!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < K , V , S : LinearMapStorage < K , V > + ? Sized > Eq for LinearMapInner < K , V , S > where K : Eq , V : PartialEq , { }
    };
}

impl_188!()