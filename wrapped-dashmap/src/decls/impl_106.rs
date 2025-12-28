macro_rules! deps {
    () => {
        MappedRef!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : Serialize > Serialize for mapref :: one :: MappedRef < 'a , K , T > { serialize_impl ! { } }
    };
}

impl_106!();