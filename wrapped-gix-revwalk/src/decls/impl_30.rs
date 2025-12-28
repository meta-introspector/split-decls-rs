macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < K : Ord , T > Eq for Item < K , T > { }
    };
}

impl_30!()