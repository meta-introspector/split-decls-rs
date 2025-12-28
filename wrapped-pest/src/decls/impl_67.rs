macro_rules! deps {
    () => {
        RuleType!();
        Tokens!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < R : RuleType > ExactSizeIterator for Tokens < '_ , R > { fn len (& self) -> usize { self . end - self . start } }
    };
}

impl_67!();