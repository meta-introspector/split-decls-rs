// Generated macro for tests_sync (module)
macro_rules! Depcrate_stream_buf_readertests_sync {
() => {
// Module: crate::stream::buf_reader
// Provides: {"tests_sync"}
// Dependencies: {}
# [cfg (test)] mod tests_sync { use super :: { BufReader , Bufferless , CombineSyncRead } ; use std :: io :: Read ; # [test] # [allow (clippy :: unused_io_amount)] fn buf_reader () { let mut read = BufReader :: with_capacity (3 , & [1u8 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 0] [..]) ; let mut buf = [0u8 ; 3] ; read . read (& mut buf) . unwrap () ; assert_eq ! (buf , [1 , 2 , 3]) ; let mut buf = [0u8 ; 3] ; read . read (& mut buf) . unwrap () ; assert_eq ! (buf , [4 , 5 , 6]) ; let mut buf = [0u8 ; 3] ; read . read (& mut buf) . unwrap () ; assert_eq ! (buf , [7 , 8 , 9]) ; let mut buf = [1u8 ; 3] ; read . read (& mut buf) . unwrap () ; assert_eq ! (buf , [0 , 1 , 1]) ; } # [test] fn buf_reader_extend_buf () { let mut read = BufReader :: with_capacity (3 , & [1u8 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 0] [..]) ; assert_eq ! (Bufferless . extend_buf_sync (& mut read) . unwrap () , 3) ; assert_eq ! (read . buffer () , [1 , 2 , 3]) ; assert_eq ! (Bufferless . extend_buf_sync (& mut read) . unwrap () , 7) ; assert_eq ! (read . buffer () , [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 0]) ; } }
};
}
