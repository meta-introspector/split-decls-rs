// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } # [test] fn test_source_chain () { let root = Error :: new (Kind :: Request , None :: < Error >) ; assert ! (root . source () . is_none ()) ; let link = super :: body (root) ; assert ! (link . source () . is_some ()) ; assert_send :: < Error > () ; assert_sync :: < Error > () ; } # [test] fn mem_size_of () { use std :: mem :: size_of ; assert_eq ! (size_of ::< Error > () , size_of ::< usize > ()) ; } # [test] fn roundtrip_io_error () { let orig = super :: request ("orig") ; let io = orig . into_io () ; let err = super :: decode_io (io) ; match err . inner . kind { Kind :: Request => () , _ => panic ! ("{err:?}") , } } # [test] fn from_unknown_io_error () { let orig = io :: Error :: new (io :: ErrorKind :: Other , "orly") ; let err = super :: decode_io (orig) ; match err . inner . kind { Kind :: Decode => () , _ => panic ! ("{err:?}") , } } # [test] fn is_timeout () { let err = super :: request (super :: TimedOut) ; assert ! (err . is_timeout ()) ; let io = io :: Error :: from (io :: ErrorKind :: TimedOut) ; let nested = super :: request (io) ; assert ! (nested . is_timeout ()) ; } }
};
}
