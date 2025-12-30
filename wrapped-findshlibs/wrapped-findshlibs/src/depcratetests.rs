// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn panic_in_each () { use std :: panic ; match panic :: catch_unwind (| | { TargetSharedLibrary :: each (| _ | panic ! ("uh oh")) ; }) { Ok (()) => panic ! ("Expected a panic, but didn't get one") , Err (any) => { assert ! (any . is ::<&'static str > () , "panic value should be a &'static str") ; assert_eq ! (* any . downcast_ref ::<&'static str > () . unwrap () , "uh oh") ; } } } # [test] fn test_load_address_bias () { TargetSharedLibrary :: each (| lib | { let svma = lib . stated_load_addr () ; let avma = lib . actual_load_addr () ; assert_eq ! (lib . avma_to_svma (avma) , svma) ; }) ; } }
};
}
