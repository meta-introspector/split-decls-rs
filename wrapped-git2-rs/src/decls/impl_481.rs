macro_rules! deps {
    () => {
        Object!();
        Binding!();
    };
}

macro_rules! impl_481 {
    () => {
        deps!();
        impl < 'repo > Clone for Object < 'repo > { fn clone (& self) -> Object < 'repo > { let mut raw = ptr :: null_mut () ; unsafe { let rc = raw :: git_object_dup (& mut raw , self . raw) ; assert_eq ! (rc , 0) ; Binding :: from_raw (raw) } } }
    };
}

impl_481!()