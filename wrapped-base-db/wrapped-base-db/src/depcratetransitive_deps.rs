// Generated macro for transitive_deps (function)
macro_rules! Depcratetransitive_deps {
() => {
// Module: crate
// Provides: {"transitive_deps"}
// Dependencies: {}
pub fn transitive_deps (db : & dyn SourceDatabase , crate_id : Crate) -> FxHashSet < Crate > { let mut worklist = vec ! [crate_id] ; let mut deps = FxHashSet :: default () ; while let Some (krate) = worklist . pop () { if ! deps . insert (krate) { continue ; } worklist . extend (krate . data (db) . dependencies . iter () . map (| dep | dep . crate_id)) ; } deps }
};
}
