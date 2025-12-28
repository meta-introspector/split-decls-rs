macro_rules! deps {
    () => {
        Trait!();
        Walkable!();
        Type!();
    };
}

macro_rules! BoundAsyncness {
    () => {
        deps!();
        # [doc = " The asyncness of a trait bound."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug)] # [derive (HashStable_Generic , Walkable)] pub enum BoundAsyncness { # [doc = " `Type: Trait`"] Normal , # [doc = " `Type: async Trait`"] Async (Span) , }
    };
}

BoundAsyncness!();