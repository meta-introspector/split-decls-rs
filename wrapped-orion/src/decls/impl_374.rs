macro_rules! deps {
    () => {
        Scalar!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl PartialEq for Scalar { fn eq (& self , other : & Self) -> bool { use subtle :: ConstantTimeEq ; self . 0 . ct_eq (& other . 0) . into () } }
    };
}

impl_374!();