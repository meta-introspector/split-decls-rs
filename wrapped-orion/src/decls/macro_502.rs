macro_rules! deps {
    () => {
        MlKem768Internal!();
    };
}

macro_rules! macro_502 {
    () => {
        deps!();
        impl_from_trait ! (SharedSecret , MlKem768Internal :: SHARED_SECRET_SIZE) ;
    };
}

macro_502!();