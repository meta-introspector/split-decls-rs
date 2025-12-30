// Generated macro for tests (module)
macro_rules! Depcrate_itertests {
() => {
// Module: crate::iter
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: DashMap ; # [test] fn iter_mut_manual_count () { let map = DashMap :: new () ; map . insert ("Johnny" , 21) ; assert_eq ! (map . len () , 1) ; let mut c = 0 ; for shard in map . shards () { c += shard . write () . iter () . count () ; } assert_eq ! (c , 1) ; } # [test] fn iter_mut_count () { let map = DashMap :: new () ; map . insert ("Johnny" , 21) ; assert_eq ! (map . len () , 1) ; assert_eq ! (map . iter_mut () . count () , 1) ; } # [test] fn iter_count () { let map = DashMap :: new () ; map . insert ("Johnny" , 21) ; assert_eq ! (map . len () , 1) ; assert_eq ! (map . iter () . count () , 1) ; } }
};
}
