macro_rules! deps {
    () => {
        Signature!();
        Binding!();
    };
}

macro_rules! impl_724 {
    () => {
        deps!();
        impl Clone for Signature < 'static > { fn clone (& self) -> Signature < 'static > { let mut raw = ptr :: null_mut () ; let rc = unsafe { raw :: git_signature_dup (& mut raw , & * self . raw) } ; assert_eq ! (rc , 0) ; unsafe { Binding :: from_raw (raw) } } }
    };
}

impl_724!();