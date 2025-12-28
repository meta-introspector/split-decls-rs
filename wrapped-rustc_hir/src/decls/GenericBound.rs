macro_rules! deps {
    () => {
        Lifetime!();
        PolyTraitRef!();
        PreciseCapturingArg!();
    };
}

macro_rules! GenericBound {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , HashStable_Generic)] pub enum GenericBound < 'hir > { Trait (PolyTraitRef < 'hir >) , Outlives (& 'hir Lifetime) , Use (& 'hir [PreciseCapturingArg < 'hir >] , Span) , }
    };
}

GenericBound!();