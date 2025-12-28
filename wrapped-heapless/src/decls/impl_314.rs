macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < A , B , LenTB , SB , const M : usize > PartialEq < VecInner < B , LenTB , SB > > for & [A ; M] where A : PartialEq < B > , LenTB : LenType , SB : VecStorage < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { (* self) . eq (other) } }
    };
}

impl_314!()