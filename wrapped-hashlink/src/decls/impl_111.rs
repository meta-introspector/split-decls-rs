macro_rules! deps {
    () => {
        FreeLink!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < K , V > Copy for FreeLink < K , V > { }
    };
}

impl_111!();