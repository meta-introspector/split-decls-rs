macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Ord for VecInner < T , LenT , S > where T : Ord , { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_325!();