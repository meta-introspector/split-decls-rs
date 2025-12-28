macro_rules! deps {
    () => {
        BoxMarker!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl ! Copy for BoxMarker { }
    };
}

impl_17!();