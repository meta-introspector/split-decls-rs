macro_rules! deps {
    () => {
        SelfKind!();
    };
}

macro_rules! ExplicitSelf {
    () => {
        deps!();
        pub type ExplicitSelf = Spanned < SelfKind > ;
    };
}

ExplicitSelf!()