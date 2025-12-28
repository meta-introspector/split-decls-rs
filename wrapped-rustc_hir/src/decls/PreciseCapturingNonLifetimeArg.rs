macro_rules! deps {
    () => {
        Node!();
        Res!();
        Lifetime!();
    };
}

macro_rules! PreciseCapturingNonLifetimeArg {
    () => {
        deps!();
        # [doc = " We need to have a [`Node`] for the [`HirId`] that we attach the type/const param"] # [doc = " resolution to. Lifetimes don't have this problem, and for them, it's actually"] # [doc = " kind of detrimental to use a custom node type versus just using [`Lifetime`],"] # [doc = " since resolve_bound_vars operates on `Lifetime`s."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct PreciseCapturingNonLifetimeArg { # [stable_hasher (ignore)] pub hir_id : HirId , pub ident : Ident , pub res : Res , }
    };
}

PreciseCapturingNonLifetimeArg!();