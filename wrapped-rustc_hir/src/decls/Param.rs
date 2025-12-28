macro_rules! deps {
    () => {
        Pat!();
    };
}

macro_rules! Param {
    () => {
        deps!();
        # [doc = " Represents a parameter in a function header."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Param < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub pat : & 'hir Pat < 'hir > , pub ty_span : Span , pub span : Span , }
    };
}

Param!()