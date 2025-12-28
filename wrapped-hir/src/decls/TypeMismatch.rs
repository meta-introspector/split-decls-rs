macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! TypeMismatch {
    () => {
        deps!();
        # [derive (Debug)] pub struct TypeMismatch < 'db > { pub expr_or_pat : InFile < ExprOrPatPtr > , pub expected : Type < 'db > , pub actual : Type < 'db > , }
    };
}

TypeMismatch!();