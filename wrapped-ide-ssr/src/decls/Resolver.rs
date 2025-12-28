macro_rules! deps {
    () => {
        ResolutionScope!();
        Placeholder!();
    };
}

macro_rules! Resolver {
    () => {
        deps!();
        struct Resolver < 'a , 'db > { resolution_scope : & 'a ResolutionScope < 'db > , placeholders_by_stand_in : FxHashMap < SmolStr , parsing :: Placeholder > , }
    };
}

Resolver!();