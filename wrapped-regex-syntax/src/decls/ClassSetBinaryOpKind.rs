macro_rules! ClassSetBinaryOpKind {
    () => {
        # [doc = " The type of a Unicode character class set operation."] # [doc = ""] # [doc = " Note that this doesn't explicitly represent union since there is no"] # [doc = " explicit union operator. Concatenation inside a character class corresponds"] # [doc = " to the union operation."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum ClassSetBinaryOpKind { # [doc = " The intersection of two sets, e.g., `\\pN&&[a-z]`."] Intersection , # [doc = " The difference of two sets, e.g., `\\pN--[0-9]`."] Difference , # [doc = " The symmetric difference of two sets. The symmetric difference is the"] # [doc = " set of elements belonging to one but not both sets."] # [doc = " e.g., `[\\pL~~[:ascii:]]`."] SymmetricDifference , }
    };
}

ClassSetBinaryOpKind!()