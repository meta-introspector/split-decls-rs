macro_rules! deps {
    () => {
        Trait!();
        Type!();
        Walkable!();
    };
}

macro_rules! BoundConstness {
    () => {
        deps!();
        # [doc = " The constness of a trait bound."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Hash)] # [derive (HashStable_Generic , Walkable)] pub enum BoundConstness { # [doc = " `Type: Trait`"] Never , # [doc = " `Type: const Trait`"] Always (Span) , # [doc = " `Type: [const] Trait`"] Maybe (Span) , }
    };
}

BoundConstness!()