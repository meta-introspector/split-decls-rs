macro_rules! deps {
    () => {
        LenType!();
        CString!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > Default for CString < N , LenT > { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_8!();