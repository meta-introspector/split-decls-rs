macro_rules! deps {
    () => {
        TyPatKind!();
    };
}

macro_rules! TyPat {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct TyPat < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : TyPatKind < 'hir > , pub span : Span , }
    };
}

TyPat!()