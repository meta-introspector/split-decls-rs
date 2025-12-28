macro_rules! deps {
    () => {
        MaxUniverse!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl MaxUniverse { fn new () -> Self { MaxUniverse { max_universe : ty :: UniverseIndex :: ROOT } } fn max_universe (self) -> ty :: UniverseIndex { self . max_universe } }
    };
}

impl_154!();