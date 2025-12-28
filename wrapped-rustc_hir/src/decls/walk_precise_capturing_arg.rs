macro_rules! deps {
    () => {
        PreciseCapturingArg!();
        Visitor!();
        Param!();
        PreciseCapturingNonLifetimeArg!();
        Lifetime!();
    };
}

macro_rules! walk_precise_capturing_arg {
    () => {
        deps!();
        pub fn walk_precise_capturing_arg < 'v , V : Visitor < 'v > > (visitor : & mut V , arg : & 'v PreciseCapturingArg < 'v > ,) -> V :: Result { match * arg { PreciseCapturingArg :: Lifetime (lt) => visitor . visit_lifetime (lt) , PreciseCapturingArg :: Param (param) => { let PreciseCapturingNonLifetimeArg { hir_id , ident , res : _ } = param ; try_visit ! (visitor . visit_id (hir_id)) ; visitor . visit_ident (ident) } } }
    };
}

walk_precise_capturing_arg!()