macro_rules! deps {
    () => {
        Walkable!();
        Type!();
        Trait!();
    };
}

macro_rules! ImplPolarity {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum ImplPolarity { # [doc = " `impl Trait for Type`"] Positive , # [doc = " `impl !Trait for Type`"] Negative (Span) , }
    };
}

ImplPolarity!();