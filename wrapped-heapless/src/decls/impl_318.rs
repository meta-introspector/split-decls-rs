macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < A , B , LenTA : LenType , SA , const N : usize > PartialEq < [B ; N] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & [B ; N]) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_318!()