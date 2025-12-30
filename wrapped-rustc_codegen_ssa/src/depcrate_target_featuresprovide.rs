// Generated macro for provide (function)
macro_rules! Depcrate_target_featuresprovide {
() => {
// Module: crate::target_features
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { rust_target_features : | tcx , cnum | { assert_eq ! (cnum , LOCAL_CRATE) ; if tcx . sess . opts . actually_rustdoc { let mut result : UnordMap < String , Stability > = Default :: default () ; for (name , stability) in rustc_target :: target_features :: all_rust_features () { use std :: collections :: hash_map :: Entry ; match result . entry (name . to_owned ()) { Entry :: Vacant (vacant_entry) => { vacant_entry . insert (stability) ; } Entry :: Occupied (mut occupied_entry) => { match (occupied_entry . get () , stability) { (Stability :: Stable , _) | (Stability :: Unstable { .. } , Stability :: Unstable { .. } | Stability :: Forbidden { .. } ,) | (Stability :: Forbidden { .. } , Stability :: Forbidden { .. }) => { } _ => { occupied_entry . insert (stability) ; } } } } } result } else { tcx . sess . target . rust_target_features () . iter () . map (| (a , b , _) | (a . to_string () , * b)) . collect () } } , implied_target_features : | tcx , feature : Symbol | { let feature = feature . as_str () ; UnordSet :: from (tcx . sess . target . implied_target_features (feature)) . into_sorted_stable_ord () . into_iter () . map (| s | Symbol :: intern (s)) . collect () } , asm_target_features , .. * providers } }
};
}
