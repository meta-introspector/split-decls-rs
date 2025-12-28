macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_764 {
    () => {
        deps!();
        impl < T : std :: fmt :: Display > From < T > for ID { fn from (value : T) -> Self { ID (value . to_string ()) } }
    };
}

impl_764!()