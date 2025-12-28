macro_rules! deps {
    () => {
        MlKem512Internal!();
    };
}

macro_rules! macro_483 {
    () => {
        deps!();
        impl_from_trait ! (SharedSecret , MlKem512Internal :: SHARED_SECRET_SIZE) ;
    };
}

macro_483!()