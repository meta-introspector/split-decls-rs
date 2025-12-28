macro_rules! deps {
    () => {
        AnonConst!();
        Safety!();
        Ty!();
    };
}

macro_rules! FieldDef {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct FieldDef < 'hir > { pub span : Span , pub vis_span : Span , pub ident : Ident , # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub ty : & 'hir Ty < 'hir > , pub safety : Safety , pub default : Option < & 'hir AnonConst > , }
    };
}

FieldDef!()