macro_rules! deps {
    () => {
        GenericParamKind!();
        GenericParamSource!();
        ParamName!();
    };
}

macro_rules! GenericParam {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct GenericParam < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub name : ParamName , pub span : Span , pub pure_wrt_drop : bool , pub kind : GenericParamKind < 'hir > , pub colon_span : Option < Span > , pub source : GenericParamSource , }
    };
}

GenericParam!();