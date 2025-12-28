macro_rules! deps {
    () => {
        PatKind!();
    };
}

macro_rules! Pat {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Pat < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : PatKind < 'hir > , pub span : Span , # [doc = " Whether to use default binding modes."] # [doc = " At present, this is false only for destructuring assignment."] pub default_binding_modes : bool , }
    };
}

Pat!();