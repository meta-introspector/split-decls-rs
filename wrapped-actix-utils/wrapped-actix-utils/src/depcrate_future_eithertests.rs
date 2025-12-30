// Generated macro for tests (module)
macro_rules! Depcrate_future_eithertests {
() => {
// Module: crate::future::either
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: future :: { ready , Ready } ; # [actix_rt :: test] async fn test_either () { let res = Either :: < _ , Ready < usize > > :: left (ready (42)) ; assert_eq ! (res . await , 42) ; let res = Either :: < Ready < usize > , _ > :: right (ready (43)) ; assert_eq ! (res . await , 43) ; } }
};
}
