macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V : Serialize > Serialize for mapref :: one :: Ref < 'a , K , V > { serialize_impl ! { } }
    };
}

impl_104!()