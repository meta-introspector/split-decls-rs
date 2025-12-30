// Generated macro for determine_lifetime_name (function)
macro_rules! Depcrate_utilsdetermine_lifetime_name {
() => {
// Module: crate::utils
// Provides: {"determine_lifetime_name"}
// Dependencies: {}
# [doc = " Determines the lifetime names. Ensure it doesn't overlap with any existing"] # [doc = " lifetime names."] pub (crate) fn determine_lifetime_name (lifetime_name : & mut String , generics : & mut Generics) { struct CollectLifetimes (Vec < String >) ; impl VisitMut for CollectLifetimes { fn visit_lifetime_param_mut (& mut self , def : & mut LifetimeParam) { self . 0 . push (def . lifetime . to_string ()) ; } } debug_assert ! (lifetime_name . starts_with ('\'')) ; let mut lifetimes = CollectLifetimes (vec ! []) ; lifetimes . visit_generics_mut (generics) ; while lifetimes . 0 . iter () . any (| name | name . starts_with (& * * lifetime_name)) { lifetime_name . push ('_') ; } }
};
}
