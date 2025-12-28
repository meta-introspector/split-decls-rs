macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! CastToUnsized {
    () => {
        deps!();
        # [derive (Debug)] pub struct CastToUnsized < 'db > { pub expr : InFile < ExprOrPatPtr > , pub cast_ty : Type < 'db > , }
    };
}

CastToUnsized!();