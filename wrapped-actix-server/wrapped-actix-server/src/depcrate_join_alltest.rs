// Generated macro for test (module)
macro_rules! Depcrate_join_alltest {
() => {
// Module: crate::join_all
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use actix_utils :: future :: ready ; use super :: * ; # [actix_rt :: test] async fn test_join_all () { let futs = vec ! [ready (Ok (1)) , ready (Err (3)) , ready (Ok (9))] ; let mut res = join_all (futs) . await . into_iter () ; assert_eq ! (Ok (1) , res . next () . unwrap ()) ; assert_eq ! (Err (3) , res . next () . unwrap ()) ; assert_eq ! (Ok (9) , res . next () . unwrap ()) ; } }
};
}
