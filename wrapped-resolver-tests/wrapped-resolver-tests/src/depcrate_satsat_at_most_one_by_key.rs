// Generated macro for sat_at_most_one_by_key (function)
macro_rules! Depcrate_satsat_at_most_one_by_key {
() => {
// Module: crate::sat
// Provides: {"sat_at_most_one_by_key"}
// Dependencies: {}
fn sat_at_most_one_by_key < K : std :: hash :: Hash + Eq > (solver : & mut varisat :: Solver < '_ > , data : impl Iterator < Item = (K , varisat :: Var) > ,) -> HashMap < K , Vec < varisat :: Var > > { let mut by_keys : HashMap < K , Vec < varisat :: Var > > = HashMap :: new () ; for (p , v) in data { by_keys . entry (p) . or_default () . push (v) } for key in by_keys . values () { sat_at_most_one (solver , key) ; } by_keys }
};
}
