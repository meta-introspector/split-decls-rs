macro_rules! deps {
    () => {
        Closure!();
        BodyOwnerKind!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl BodyOwnerKind { pub fn is_fn_or_closure (self) -> bool { match self { BodyOwnerKind :: Fn | BodyOwnerKind :: Closure => true , BodyOwnerKind :: Const { .. } | BodyOwnerKind :: Static (_) | BodyOwnerKind :: GlobalAsm => { false } } } }
    };
}

impl_212!();