macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < A , B , LenTA , SA > PartialEq < [B] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & [B]) -> bool { self . as_slice () . eq (other) } }
    };
}

impl_320!();