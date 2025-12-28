macro_rules! deps {
    () => {
        CfgExpr!();
        Literal!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Literal { fn new (expr : CfgExpr) -> Self { match expr { CfgExpr :: Invalid => Self { negate : false , var : None } , CfgExpr :: Atom (atom) => Self { negate : false , var : Some (atom) } , CfgExpr :: Not (expr) => match * expr { CfgExpr :: Invalid => Self { negate : true , var : None } , CfgExpr :: Atom (atom) => Self { negate : true , var : Some (atom) } , _ => unreachable ! ("non-atom {:?}" , expr) , } , CfgExpr :: Any (_) | CfgExpr :: All (_) => unreachable ! ("non-literal {:?}" , expr) , } } }
    };
}

impl_17!()