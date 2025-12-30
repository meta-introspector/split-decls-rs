// Generated macro for tests (module)
macro_rules! Depcrate_channeltests {
() => {
// Module: crate::channel
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: available_buffer_size ; use crate :: consts :: BUF_SIZE ; # [test] fn test_rtt_available_buffer_size () { let avail = | read : usize , write : usize | available_buffer_size (read , write) ; assert_eq ! (avail (0 , 0) , BUF_SIZE - 1) ; assert_eq ! (avail (10 , 10) , BUF_SIZE - 1) ; assert_eq ! (avail (BUF_SIZE - 1 , BUF_SIZE - 1) , BUF_SIZE - 1) ; assert_eq ! (avail (0 , BUF_SIZE - 1) , 0) ; assert_eq ! (avail (5 , 4) , 0) ; assert_eq ! (avail (1 , 0) , 0) ; assert_eq ! (avail (10 , 5) , 10 - 5 - 1) ; assert_eq ! (avail (BUF_SIZE - 1 , 0) , (BUF_SIZE - 1) - 0 - 1) ; assert_eq ! (avail (5 , 10) , BUF_SIZE - 10 - 1 + 5) ; assert_eq ! (avail (0 , 1) , BUF_SIZE - 1 - 1 + 0) ; assert_eq ! (avail (1 , BUF_SIZE - 1) , BUF_SIZE - (BUF_SIZE - 1) - 1 + 1) ; assert_eq ! (avail (1 , BUF_SIZE - 1) , 1) ; assert_eq ! (avail (2 , BUF_SIZE - 1) , 2) ; let data_in_buffer = | read : usize , write : usize | (write + BUF_SIZE - read) % BUF_SIZE ; let free_should_be = | read : usize , write : usize | BUF_SIZE - 1 - data_in_buffer (read , write) ; for read in 0 .. BUF_SIZE . min (64) { for write in 0 .. BUF_SIZE . min (64) { let expected = free_should_be (read , write) ; let actual = avail (read , write) ; assert_eq ! (actual , expected , "Mismatch at read={read}, write={write}") ; } } } }
};
}
