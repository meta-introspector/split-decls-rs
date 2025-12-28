macro_rules! deps {
    () => {
        BoxMarker!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ! Clone for BoxMarker { }
    };
}

impl_16!();