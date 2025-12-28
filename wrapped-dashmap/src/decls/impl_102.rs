macro_rules! deps {
    () => {
        RefMulti!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V : Serialize > Serialize for mapref :: multiple :: RefMulti < 'a , K , V > { serialize_impl ! { } }
    };
}

impl_102!()