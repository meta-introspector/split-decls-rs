macro_rules! deps {
    () => {
        MlKem512Internal!();
    };
}

macro_rules! macro_485 {
    () => {
        deps!();
        impl_from_trait ! (Ciphertext , MlKem512Internal :: CIPHERTEXT_SIZE) ;
    };
}

macro_485!()