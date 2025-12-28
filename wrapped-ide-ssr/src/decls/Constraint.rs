macro_rules! deps {
    () => {
        NodeKind!();
    };
}

macro_rules! Constraint {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum Constraint { Kind (NodeKind) , Not (Box < Constraint >) , }
    };
}

Constraint!();