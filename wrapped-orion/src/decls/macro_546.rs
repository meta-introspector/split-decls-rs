macro_rules! deps {
    () => {
        EncapsulationKey!();
    };
}

macro_rules! macro_546 {
    () => {
        deps!();
        impl_from_trait ! (EncapsulationKey , PUBLIC_KEY_SIZE) ;
    };
}

macro_546!();