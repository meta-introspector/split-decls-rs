macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < A , B , LenTB : LenType , SB : VecStorage < B > > PartialEq < VecInner < B , LenTB , SB > > for & mut [A] where A : PartialEq < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { (* * self) . eq (other) } }
    };
}

impl_317!();