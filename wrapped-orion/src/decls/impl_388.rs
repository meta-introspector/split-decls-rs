macro_rules! deps {
    () => {
        PrivateKey!();
        Scalar!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl From < [u8 ; PRIVATE_KEY_SIZE] > for PrivateKey { # [inline] fn from (bytes : [u8 ; PRIVATE_KEY_SIZE]) -> Self { PrivateKey { scalar : Scalar :: from_slice (bytes . as_ref ()) . unwrap () , } } }
    };
}

impl_388!()