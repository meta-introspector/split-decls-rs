// Generated macro for tests (module)
macro_rules! Depcrate_common_io_rewindtests {
() => {
// Module: crate::common::io::rewind
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2") ,))] # [cfg (test)] mod tests { use super :: super :: Compat ; use super :: Rewind ; use bytes :: Bytes ; use tokio :: io :: AsyncReadExt ; # [cfg (not (miri))] # [tokio :: test] async fn partial_rewind () { let underlying = [104 , 101 , 108 , 108 , 111] ; let mock = tokio_test :: io :: Builder :: new () . read (& underlying) . build () ; let mut stream = Compat :: new (Rewind :: new (Compat :: new (mock))) ; let mut buf = [0 ; 2] ; stream . read_exact (& mut buf) . await . expect ("read1") ; stream . 0 . rewind (Bytes :: copy_from_slice (& buf [..])) ; let mut buf = [0 ; 5] ; stream . read_exact (& mut buf) . await . expect ("read1") ; assert_eq ! (& buf , & underlying) ; } # [cfg (not (miri))] # [tokio :: test] async fn full_rewind () { let underlying = [104 , 101 , 108 , 108 , 111] ; let mock = tokio_test :: io :: Builder :: new () . read (& underlying) . build () ; let mut stream = Compat :: new (Rewind :: new (Compat :: new (mock))) ; let mut buf = [0 ; 5] ; stream . read_exact (& mut buf) . await . expect ("read1") ; stream . 0 . rewind (Bytes :: copy_from_slice (& buf [..])) ; let mut buf = [0 ; 5] ; stream . read_exact (& mut buf) . await . expect ("read1") ; assert_eq ! (& buf , & underlying) ; } }
};
}
