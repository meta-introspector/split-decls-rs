// Generated macro for find_lifetimes_in_tpb (function)
macro_rules! Depcratefind_lifetimes_in_tpb {
() => {
// Module: crate
// Provides: {"find_lifetimes_in_tpb"}
// Dependencies: {}
fn find_lifetimes_in_tpb (bound : & TypeParamBound) -> HashSet < Lifetime > { let mut ret = HashSet :: default () ; match bound { TypeParamBound :: Lifetime (lt) => { ret . insert (lt . clone ()) ; } , TypeParamBound :: Trait (tb) => { ret . extend (find_lifetimes_in_path (& tb . path)) ; } , _ => () } ; ret }
};
}
