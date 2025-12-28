macro_rules! deps {
    () => {
        MlKem768Internal!();
    };
}

macro_rules! macro_504 {
    () => {
        deps!();
        impl_from_trait ! (Ciphertext , MlKem768Internal :: CIPHERTEXT_SIZE) ;
    };
}

macro_504!();