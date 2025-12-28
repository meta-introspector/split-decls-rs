macro_rules! deps {
    () => {
        ExprId!();
    };
}

macro_rules! RecordLitField {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct RecordLitField { pub name : Name , pub expr : ExprId , }
    };
}

RecordLitField!();