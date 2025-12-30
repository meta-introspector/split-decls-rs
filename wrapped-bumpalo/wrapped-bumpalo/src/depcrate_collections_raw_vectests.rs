// Generated macro for tests (module)
macro_rules! Depcrate_collections_raw_vectests {
() => {
// Module: crate::collections::raw_vec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reserve_does_not_overallocate () { let bump = Bump :: new () ; { let mut v : RawVec < u32 > = RawVec :: new_in (& bump) ; v . reserve (0 , 9) ; assert_eq ! (9 , v . cap ()) ; } { let mut v : RawVec < u32 > = RawVec :: new_in (& bump) ; v . reserve (0 , 7) ; assert_eq ! (7 , v . cap ()) ; v . reserve (7 , 90) ; assert_eq ! (97 , v . cap ()) ; } { let mut v : RawVec < u32 > = RawVec :: new_in (& bump) ; v . reserve (0 , 12) ; assert_eq ! (12 , v . cap ()) ; v . reserve (12 , 3) ; assert ! (v . cap () >= 12 + 12 / 2) ; } } }
};
}
