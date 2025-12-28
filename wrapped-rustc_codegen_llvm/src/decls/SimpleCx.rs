macro_rules! deps {
    () => {
        SCx!();
        GenericCx!();
    };
}

macro_rules! SimpleCx {
    () => {
        deps!();
        pub (crate) type SimpleCx < 'll > = GenericCx < 'll , SCx < 'll > > ;
    };
}

SimpleCx!();