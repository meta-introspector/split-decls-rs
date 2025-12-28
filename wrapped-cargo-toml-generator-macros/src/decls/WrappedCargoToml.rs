macro_rules! WrappedCargoToml {
    () => {
        struct WrappedCargoToml (CargoToml) ;
    };
}

WrappedCargoToml!()