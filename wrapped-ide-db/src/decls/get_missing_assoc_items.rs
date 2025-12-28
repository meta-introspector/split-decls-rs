macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! get_missing_assoc_items {
    () => {
        deps!();
        # [doc = " Given the `impl` block, returns the list of associated items (e.g. functions or types) that are"] # [doc = " missing in this `impl` block."] pub fn get_missing_assoc_items (sema : & Semantics < '_ , RootDatabase > , impl_def : & ast :: Impl ,) -> Vec < hir :: AssocItem > { let imp = match sema . to_def (impl_def) { Some (it) => it , None => return vec ! [] , } ; let mut impl_fns_consts = FxHashSet :: default () ; let mut impl_type = FxHashSet :: default () ; let edition = imp . module (sema . db) . krate () . edition (sema . db) ; for item in imp . items (sema . db) { match item { hir :: AssocItem :: Function (it) => { impl_fns_consts . insert (it . name (sema . db) . display (sema . db , edition) . to_string ()) ; } hir :: AssocItem :: Const (it) => { if let Some (name) = it . name (sema . db) { impl_fns_consts . insert (name . display (sema . db , edition) . to_string ()) ; } } hir :: AssocItem :: TypeAlias (it) => { impl_type . insert (it . name (sema . db) . display (sema . db , edition) . to_string ()) ; } } } resolve_target_trait (sema , impl_def) . map_or (vec ! [] , | target_trait | { target_trait . items (sema . db) . into_iter () . filter (| i | match i { hir :: AssocItem :: Function (f) => ! impl_fns_consts . contains (& f . name (sema . db) . display (sema . db , edition) . to_string ()) , hir :: AssocItem :: TypeAlias (t) => { ! impl_type . contains (& t . name (sema . db) . display (sema . db , edition) . to_string ()) } hir :: AssocItem :: Const (c) => c . name (sema . db) . map (| n | ! impl_fns_consts . contains (& n . display (sema . db , edition) . to_string ())) . unwrap_or_default () , }) . collect () }) }
    };
}

get_missing_assoc_items!();