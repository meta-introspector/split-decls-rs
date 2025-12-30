// Generated macro for tests (module)
macro_rules! Depcrate_arrayvectests {
() => {
// Module: crate::arrayvec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use super :: ArrayVec ; use crate :: ErrorKind ; # [test] fn add () { let mut vec = ArrayVec :: < u8 , 3 > :: new () ; vec . push (1) . unwrap () ; vec . push (2) . unwrap () ; vec . push (3) . unwrap () ; assert_eq ! (vec . push (4) . err () . unwrap () , ErrorKind :: Overlength . into ()) ; assert_eq ! (vec . len () , 3) ; } }
};
}
