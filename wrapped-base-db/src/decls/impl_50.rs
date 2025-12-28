macro_rules! deps {
    () => {
        RootQueryDb!();
        Crate!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Crate { # [doc = " Returns an iterator over all transitive dependencies of the given crate,"] # [doc = " including the crate itself."] # [doc = ""] # [doc = " **Warning**: do not use this query in `hir-*` crates! It kills incrementality across crate metadata modifications."] pub fn transitive_deps (self , db : & dyn salsa :: Database) -> Box < [Crate] > { let mut worklist = vec ! [self] ; let mut deps_seen = FxHashSet :: default () ; let mut deps = Vec :: new () ; while let Some (krate) = worklist . pop () { if ! deps_seen . insert (krate) { continue ; } deps . push (krate) ; worklist . extend (krate . data (db) . dependencies . iter () . map (| dep | dep . crate_id)) ; } deps . into_boxed_slice () } # [doc = " Returns all transitive reverse dependencies of the given crate,"] # [doc = " including the crate itself."] # [doc = ""] # [doc = " **Warning**: do not use this query in `hir-*` crates! It kills incrementality across crate metadata modifications."] pub fn transitive_rev_deps (self , db : & dyn RootQueryDb) -> Box < [Crate] > { let mut worklist = vec ! [self] ; let mut rev_deps = FxHashSet :: default () ; rev_deps . insert (self) ; let mut inverted_graph = FxHashMap :: < _ , Vec < _ > > :: default () ; db . all_crates () . iter () . for_each (| & krate | { krate . data (db) . dependencies . iter () . for_each (| dep | inverted_graph . entry (dep . crate_id) . or_default () . push (krate)) }) ; while let Some (krate) = worklist . pop () { if let Some (crate_rev_deps) = inverted_graph . get (& krate) { crate_rev_deps . iter () . copied () . filter (| & rev_dep | rev_deps . insert (rev_dep)) . for_each (| rev_dep | worklist . push (rev_dep)) ; } } rev_deps . into_iter () . collect :: < Box < _ > > () } }
    };
}

impl_50!();