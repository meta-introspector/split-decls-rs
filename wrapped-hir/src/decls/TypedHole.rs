macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! TypedHole {
    () => {
        deps!();
        # [derive (Debug)] pub struct TypedHole < 'db > { pub expr : InFile < ExprOrPatPtr > , pub expected : Type < 'db > , }
    };
}

TypedHole!();