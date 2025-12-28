macro_rules! deps {
    () => {
        PatId!();
        ExprId!();
    };
}

macro_rules! MatchArm {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct MatchArm { pub pat : PatId , pub guard : Option < ExprId > , pub expr : ExprId , }
    };
}

MatchArm!();