macro_rules! deps {
    () => {
        GenericArg!();
        AssocItemConstraint!();
        Walkable!();
    };
}

macro_rules! AngleBracketedArg {
    () => {
        deps!();
        # [doc = " Either an argument for a generic parameter or a constraint on an associated item."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum AngleBracketedArg { # [doc = " A generic argument for a generic parameter."] Arg (GenericArg) , # [doc = " A constraint on an associated item."] Constraint (AssocItemConstraint) , }
    };
}

AngleBracketedArg!()