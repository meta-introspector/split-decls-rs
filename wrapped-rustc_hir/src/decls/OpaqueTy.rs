macro_rules! deps {
    () => {
        OpaqueTyOrigin!();
        GenericBounds!();
    };
}

macro_rules! OpaqueTy {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct OpaqueTy < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub bounds : GenericBounds < 'hir > , pub origin : OpaqueTyOrigin < LocalDefId > , pub span : Span , }
    };
}

OpaqueTy!();