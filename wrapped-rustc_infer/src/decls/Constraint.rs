macro_rules! deps {
    () => {
        ConstraintKind!();
    };
}

macro_rules! Constraint {
    () => {
        deps!();
        # [doc = " Represents a constraint that influences the inference process."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub struct Constraint < 'tcx > { pub kind : ConstraintKind , pub sub : Region < 'tcx > , pub sup : Region < 'tcx > , }
    };
}

Constraint!();