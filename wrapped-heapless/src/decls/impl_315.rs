macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < A , B , LenTB , SB > PartialEq < VecInner < B , LenTB , SB > > for [A] where A : PartialEq < B > , LenTB : LenType , SB : VecStorage < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { self . eq (other . as_slice ()) } }
    };
}

impl_315!()