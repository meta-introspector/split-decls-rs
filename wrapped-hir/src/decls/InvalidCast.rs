macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! InvalidCast {
    () => {
        deps!();
        # [derive (Debug)] pub struct InvalidCast < 'db > { pub expr : InFile < ExprOrPatPtr > , pub error : CastError , pub expr_ty : Type < 'db > , pub cast_ty : Type < 'db > , }
    };
}

InvalidCast!()