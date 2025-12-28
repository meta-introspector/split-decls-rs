macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_527 {
    () => {
        deps!();
        impl Ord for Oid { fn cmp (& self , other : & Oid) -> Ordering { c_cmp_to_ordering (unsafe { raw :: git_oid_cmp (& self . raw , & other . raw) }) } }
    };
}

impl_527!()