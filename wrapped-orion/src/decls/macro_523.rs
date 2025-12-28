macro_rules! deps {
    () => {
        MlKem1024Internal!();
    };
}

macro_rules! macro_523 {
    () => {
        deps!();
        impl_from_trait ! (Ciphertext , MlKem1024Internal :: CIPHERTEXT_SIZE) ;
    };
}

macro_523!();