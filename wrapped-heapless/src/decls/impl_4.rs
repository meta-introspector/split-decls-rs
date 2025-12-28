macro_rules! deps {
    () => {
        LenType!();
        CString!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < const N : usize , LenT : LenType > Zeroize for CString < N , LenT > { fn zeroize (& mut self) { self . inner . zeroize () ; const { assert ! (N > 0) ; } unsafe { self . inner . push_unchecked (b'\0') } ; } }
    };
}

impl_4!();