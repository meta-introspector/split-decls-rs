macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl < T , LenTA : LenType , LenTB : LenType , SA : VecStorage < T > + ? Sized , SB : VecStorage < T > + ? Sized > PartialOrd < VecInner < T , LenTA , SA > > for VecInner < T , LenTB , SB > where T : PartialOrd , { fn partial_cmp (& self , other : & VecInner < T , LenTA , SA >) -> Option < Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
    };
}

impl_324!()