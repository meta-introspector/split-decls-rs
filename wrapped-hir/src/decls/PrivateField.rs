macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! PrivateField {
    () => {
        deps!();
        # [derive (Debug)] pub struct PrivateField { pub expr : InFile < ExprOrPatPtr > , pub field : Field , }
    };
}

PrivateField!();