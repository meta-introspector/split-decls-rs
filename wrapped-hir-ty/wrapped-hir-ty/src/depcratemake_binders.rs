// Generated macro for make_binders (function)
macro_rules! Depcratemake_binders {
() => {
// Module: crate
// Provides: {"make_binders"}
// Dependencies: {}
pub (crate) fn make_binders < T : HasInterner < Interner = Interner > > (db : & dyn HirDatabase , generics : & Generics , value : T ,) -> Binders < T > { Binders :: new (variable_kinds_from_iter (db , generics . iter_id ()) , value) }
};
}
