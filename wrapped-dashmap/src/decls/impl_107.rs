macro_rules! deps {
    () => {
        MappedRefMut!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : Serialize > Serialize for mapref :: one :: MappedRefMut < 'a , K , T > { serialize_impl ! { } }
    };
}

impl_107!();