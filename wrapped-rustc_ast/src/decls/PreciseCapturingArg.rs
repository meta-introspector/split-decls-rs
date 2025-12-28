macro_rules! deps {
    () => {
        GenericArg!();
        Walkable!();
        Lifetime!();
        LifetimeCtxt!();
        Type!();
        Path!();
    };
}

macro_rules! PreciseCapturingArg {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum PreciseCapturingArg { # [doc = " Lifetime parameter."] Lifetime (# [visitable (extra = LifetimeCtxt :: GenericArg)] Lifetime) , # [doc = " Type or const parameter."] Arg (Path , NodeId) , }
    };
}

PreciseCapturingArg!()