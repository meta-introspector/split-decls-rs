macro_rules! deps {
    () => {
        GenericBuilder!();
        SCx!();
    };
}

macro_rules! SBuilder {
    () => {
        deps!();
        pub (crate) type SBuilder < 'a , 'll > = GenericBuilder < 'a , 'll , SCx < 'll > > ;
    };
}

SBuilder!();