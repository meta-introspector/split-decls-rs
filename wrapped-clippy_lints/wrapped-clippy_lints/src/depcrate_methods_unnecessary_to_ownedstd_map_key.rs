// Generated macro for std_map_key (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedstd_map_key {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"std_map_key"}
// Dependencies: {}
fn std_map_key < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { match ty . kind () { ty :: Adt (adt , args) if matches ! (cx . tcx . get_diagnostic_name (adt . did ()) , Some (sym :: BTreeMap | sym :: BTreeSet | sym :: HashMap | sym :: HashSet)) => { Some (args . type_at (0)) } , _ => None , } }
};
}
