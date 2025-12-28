macro_rules! deps {
    () => {
        ByRef!();
        Walkable!();
    };
}

macro_rules! BindingMode {
    () => {
        deps!();
        # [doc = " The mode of a binding (`mut`, `ref mut`, etc)."] # [doc = " Used for both the explicit binding annotations given in the HIR for a binding"] # [doc = " and the final binding mode that we infer after type inference/match ergonomics."] # [doc = " `.0` is the by-reference mode (`ref`, `ref mut`, or by value),"] # [doc = " `.1` is the mutability of the binding."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [derive (Encodable , Decodable , HashStable_Generic , Walkable)] pub struct BindingMode (pub ByRef , pub Mutability) ;
    };
}

BindingMode!();