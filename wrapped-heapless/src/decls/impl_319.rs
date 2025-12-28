macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < A , B , LenTA , SA , const N : usize > PartialEq < & [B ; N] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & & [B ; N]) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_319!();