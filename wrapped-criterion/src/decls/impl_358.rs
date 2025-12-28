macro_rules! deps {
    () => {
        Float!();
        LabeledSample!();
        Sample!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl < 'a , A > Deref for LabeledSample < 'a , A > where A : Float , { type Target = Sample < A > ; fn deref (& self) -> & Sample < A > { self . sample } }
    };
}

impl_358!();