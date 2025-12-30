// Generated macro for tests (module)
macro_rules! Depcrate_atomictests {
() => {
// Module: crate::atomic
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , not (crossbeam_loom)))] mod tests { use super :: { Owned , Shared } ; use std :: mem :: MaybeUninit ; # [test] fn valid_tag_i8 () { Shared :: < i8 > :: null () . with_tag (0) ; } # [test] fn valid_tag_i64 () { Shared :: < i64 > :: null () . with_tag (7) ; } # [test] fn const_atomic_null () { use super :: Atomic ; static _U : Atomic < u8 > = Atomic :: < u8 > :: null () ; } # [test] fn array_init () { let owned = Owned :: < [MaybeUninit < usize >] > :: init (10) ; let arr : & [MaybeUninit < usize >] = & owned ; assert_eq ! (arr . len () , 10) ; } }
};
}
