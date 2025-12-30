// Generated macro for quoting_strategy (function)
macro_rules! Depcrate_bytesquoting_strategy {
() => {
// Module: crate::bytes
// Provides: {"quoting_strategy"}
// Dependencies: {}
# [doc = " Given an input, return a quoting strategy that can cover some prefix of the string, along with"] # [doc = " the size of that prefix."] # [doc = ""] # [doc = " Precondition: input size is nonzero.  (Empty strings are handled by the caller.)"] # [doc = " Postcondition: returned size is nonzero."] # [inline (never)] fn quoting_strategy (in_bytes : & [u8]) -> (usize , QuotingStrategy) { const UNQUOTED_OK : u8 = 1 ; const SINGLE_QUOTED_OK : u8 = 2 ; const DOUBLE_QUOTED_OK : u8 = 4 ; let mut prev_ok = SINGLE_QUOTED_OK | DOUBLE_QUOTED_OK | UNQUOTED_OK ; let mut i = 0 ; if in_bytes [0] == b'^' { prev_ok = SINGLE_QUOTED_OK ; i = 1 ; } while i < in_bytes . len () { let c = in_bytes [i] ; let mut cur_ok = prev_ok ; if c >= 0x80 { cur_ok &= ! UNQUOTED_OK ; } else { if ! unquoted_ok_fast (c) { cur_ok &= ! UNQUOTED_OK ; } if ! single_quoted_ok (c) { cur_ok &= ! SINGLE_QUOTED_OK ; } if ! double_quoted_ok (c) { cur_ok &= ! DOUBLE_QUOTED_OK ; } } if cur_ok == 0 { break ; } prev_ok = cur_ok ; i += 1 ; } let strategy = if prev_ok & UNQUOTED_OK != 0 { QuotingStrategy :: Unquoted } else if prev_ok & SINGLE_QUOTED_OK != 0 { QuotingStrategy :: SingleQuoted } else if prev_ok & DOUBLE_QUOTED_OK != 0 { QuotingStrategy :: DoubleQuoted } else { unreachable ! () } ; debug_assert ! (i > 0) ; (i , strategy) }
};
}
