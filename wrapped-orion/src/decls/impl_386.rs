macro_rules! deps {
    () => {
        PrivateKey!();
        Scalar!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl PartialEq < & [u8] > for PrivateKey { fn eq (& self , other : & & [u8]) -> bool { match Scalar :: from_slice (other) { Ok (other_scalar) => self . scalar == other_scalar , Err (_) => false , } } }
    };
}

impl_386!();