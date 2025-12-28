macro_rules! deps {
    () => {
        NonSubstitution!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl PrefixHandle { fn is_template_prefix (& self) -> bool { match * self { PrefixHandle :: BackReference (_) | PrefixHandle :: WellKnown (_) => true , PrefixHandle :: NonSubstitution (_) => false , } } }
    };
}

impl_110!()