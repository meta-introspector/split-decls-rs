macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'a , V : Hash + Eq + Serialize > Serialize for setref :: one :: Ref < 'a , V > { serialize_impl ! { } }
    };
}

impl_109!()