// Generated macro for round_up_to_unchecked (function)
macro_rules! Depcrateround_up_to_unchecked {
() => {
// Module: crate
// Provides: {"round_up_to_unchecked"}
// Dependencies: {}
# [doc = " Like `round_up_to` but turns overflow into undefined behavior rather than"] # [doc = " returning `None`."] # [inline] pub (crate) unsafe fn round_up_to_unchecked (n : usize , divisor : usize) -> usize { match round_up_to (n , divisor) { Some (x) => x , None => { debug_assert ! (false , "round_up_to_unchecked failed") ; core :: hint :: unreachable_unchecked () } } }
};
}
