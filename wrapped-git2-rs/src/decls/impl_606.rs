macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_606 {
    () => {
        deps!();
        impl < 'repo > Ord for Reference < 'repo > { fn cmp (& self , other : & Reference < 'repo >) -> Ordering { c_cmp_to_ordering (unsafe { raw :: git_reference_cmp (& * self . raw , & * other . raw) }) } }
    };
}

impl_606!();