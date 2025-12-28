macro_rules! deps {
    () => {
        RefMut!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V : Serialize > Serialize for mapref :: one :: RefMut < 'a , K , V > { serialize_impl ! { } }
    };
}

impl_105!()