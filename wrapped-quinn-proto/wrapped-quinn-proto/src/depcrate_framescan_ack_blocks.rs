// Generated macro for scan_ack_blocks (function)
macro_rules! Depcrate_framescan_ack_blocks {
() => {
// Module: crate::frame
// Provides: {"scan_ack_blocks"}
// Dependencies: {}
# [doc = " Validate exactly `n` ACK ranges in `buf` and return the number of bytes they cover"] fn scan_ack_blocks (mut buf : & [u8] , largest : u64 , n : usize) -> Result < usize , IterErr > { let total_len = buf . remaining () ; let first_block = buf . get_var () ? ; let mut smallest = largest . checked_sub (first_block) . ok_or (IterErr :: Malformed) ? ; for _ in 0 .. n { let gap = buf . get_var () ? ; smallest = smallest . checked_sub (gap + 2) . ok_or (IterErr :: Malformed) ? ; let block = buf . get_var () ? ; smallest = smallest . checked_sub (block) . ok_or (IterErr :: Malformed) ? ; } Ok (total_len - buf . remaining ()) }
};
}
