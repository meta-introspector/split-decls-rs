macro_rules! deps {
    () => {
        PartialConstStability!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl PartialConstStability { pub fn is_const_unstable (& self) -> bool { self . level . is_unstable () } pub fn is_const_stable (& self) -> bool { self . level . is_stable () } }
    };
}

impl_449!()