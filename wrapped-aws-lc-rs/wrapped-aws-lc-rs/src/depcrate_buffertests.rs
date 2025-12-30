// Generated macro for tests (module)
macro_rules! Depcrate_buffertests {
() => {
// Module: crate::buffer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_new () { let buffer : Buffer < u8 > = Buffer :: new (vec ! [1 , 2 , 3]) ; assert_eq ! (buffer . as_ref () , & [1 , 2 , 3]) ; } # [test] fn test_take_from_slice () { let mut slice = [1 , 2 , 3] ; let buffer : Buffer < u8 > = Buffer :: take_from_slice (& mut slice) ; assert_eq ! (buffer . as_ref () , & [1 , 2 , 3]) ; assert_eq ! (slice , [0 , 0 , 0]) ; } }
};
}
