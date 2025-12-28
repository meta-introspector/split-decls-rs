macro_rules! deps {
    () => {
        ResolvedPattern!();
    };
}

macro_rules! ResolvedRule {
    () => {
        deps!();
        pub (crate) struct ResolvedRule < 'db > { pub (crate) pattern : ResolvedPattern < 'db > , pub (crate) template : Option < ResolvedPattern < 'db > > , pub (crate) index : usize , }
    };
}

ResolvedRule!();