macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < A , B , LenTA , SA > PartialEq < & mut [B] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & & mut [B]) -> bool { self . as_slice () . eq (* other) } }
    };
}

impl_322!();