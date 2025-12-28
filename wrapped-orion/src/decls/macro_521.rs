macro_rules! deps {
    () => {
        MlKem1024Internal!();
    };
}

macro_rules! macro_521 {
    () => {
        deps!();
        impl_from_trait ! (SharedSecret , MlKem1024Internal :: SHARED_SECRET_SIZE) ;
    };
}

macro_521!();