macro_rules! deps {
    () => {
        BoundConstness!();
        BoundPolarity!();
        Walkable!();
        BoundAsyncness!();
    };
}

macro_rules! TraitBoundModifiers {
    () => {
        deps!();
        # [doc = " Modifiers on a trait bound like `[const]`, `?` and `!`."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Walkable)] pub struct TraitBoundModifiers { pub constness : BoundConstness , pub asyncness : BoundAsyncness , pub polarity : BoundPolarity , }
    };
}

TraitBoundModifiers!();