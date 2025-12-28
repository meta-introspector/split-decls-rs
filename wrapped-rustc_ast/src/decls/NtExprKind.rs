macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! NtExprKind {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum NtExprKind { Expr , Expr2021 { inferred : bool } , }
    };
}

NtExprKind!();