macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < A , B , LenTA , LenTB , SA , SB > PartialEq < VecInner < B , LenTB , SB > > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , LenTB : LenType , SA : VecStorage < A > + ? Sized , SB : VecStorage < B > + ? Sized , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_312!();