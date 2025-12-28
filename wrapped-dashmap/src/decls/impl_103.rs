macro_rules! deps {
    () => {
        RefMutMulti!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V : Serialize > Serialize for mapref :: multiple :: RefMutMulti < 'a , K , V > { serialize_impl ! { } }
    };
}

impl_103!();