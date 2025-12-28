macro_rules! deps {
    () => {
        LifetimeCtxt!();
        PreciseCapturingArg!();
        Lifetime!();
        Trait!();
        Walkable!();
        PolyTraitRef!();
    };
}

macro_rules! GenericBound {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericBound { Trait (PolyTraitRef) , Outlives (# [visitable (extra = LifetimeCtxt :: Bound)] Lifetime) , # [doc = " Precise capturing syntax: `impl Sized + use<'a>`"] Use (ThinVec < PreciseCapturingArg > , Span) , }
    };
}

GenericBound!();