macro_rules! deps {
    () => {
        RefMulti!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'a , V : Hash + Eq + Serialize > Serialize for setref :: multiple :: RefMulti < 'a , V > { serialize_impl ! { } }
    };
}

impl_108!();