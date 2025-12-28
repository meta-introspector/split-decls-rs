macro_rules! deps {
    () => {
        Place!();
        ProjectionId!();
        LocalId!();
    };
}

macro_rules! impl_879 {
    () => {
        deps!();
        impl < 'db > From < LocalId < 'db > > for Place < 'db > { fn from (local : LocalId < 'db >) -> Self { Self { local , projection : ProjectionId :: EMPTY } } }
    };
}

impl_879!();