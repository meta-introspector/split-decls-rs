macro_rules! deps {
    () => {
        PatId!();
        ExprId!();
    };
}

macro_rules! ExprOrPatId {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum ExprOrPatId { ExprId (ExprId) , PatId (PatId) , }
    };
}

ExprOrPatId!()