macro_rules! deps {
    () => {
        PasswordHash!();
    };
}

macro_rules! macro_638 {
    () => {
        deps!();
        impl_ct_partialeq_trait ! (PasswordHash , unprotected_as_bytes) ;
    };
}

macro_638!()