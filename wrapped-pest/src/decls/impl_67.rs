macro_rules! deps {
    () => {
        Tokens!();
        RuleType!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < R : RuleType > ExactSizeIterator for Tokens < '_ , R > { fn len (& self) -> usize { self . end - self . start } }
    };
}

impl_67!()