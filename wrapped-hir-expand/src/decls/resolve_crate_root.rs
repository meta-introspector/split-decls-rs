macro_rules! deps {
    () => {
        ExpandDatabase!();
    };
}

macro_rules! resolve_crate_root {
    () => {
        deps!();
        pub fn resolve_crate_root (db : & dyn ExpandDatabase , mut ctxt : SyntaxContext) -> Option < Crate > { ctxt = ctxt . normalize_to_macro_rules (db) ; let mut iter = ctxt . marks_rev (db) . peekable () ; let mut result_mark = None ; while let Some (& (mark , Transparency :: Opaque)) = iter . peek () { result_mark = Some (mark) ; iter . next () ; } while let Some ((mark , Transparency :: SemiTransparent)) = iter . next () { result_mark = Some (mark) ; } result_mark . map (| call | db . lookup_intern_macro_call (call . into ()) . def . krate) }
    };
}

resolve_crate_root!();