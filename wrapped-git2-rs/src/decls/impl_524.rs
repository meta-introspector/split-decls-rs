macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl PartialEq for Oid { fn eq (& self , other : & Oid) -> bool { unsafe { raw :: git_oid_equal (& self . raw , & other . raw) != 0 } } }
    };
}

impl_524!()