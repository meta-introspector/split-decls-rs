// Generated macro for impl_121 (impl)
macro_rules! Depcrate_timezone_implimpl_121 {
() => {
// Module: crate::timezone_impl
// Provides: {"impl_121"}
// Dependencies: {}
impl GapInfo { # [doc = " Return information about a gap."] # [doc = ""] # [doc = " It returns `None` if `local` is not in a gap for the current timezone."] # [doc = ""] # [doc = " If `local` is at the limits of the known timestamps the fields `begin` or `end` in"] # [doc = " [`GapInfo`] will be `None`."] pub fn new (local : & NaiveDateTime , tz : & Tz) -> Option < Self > { let timestamp = local . and_utc () . timestamp () ; let timespans = tz . timespans () ; let index = binary_search (0 , timespans . len () , | i | { timespans . local_span (i) . cmp (timestamp) }) ; let Err (end_idx) = index else { return None ; } ; let begin = match end_idx { 0 => None , _ => { let start_idx = end_idx - 1 ; timespans . local_span (start_idx) . end . and_then (| start_time | DateTime :: from_timestamp (start_time , 0)) . map (| start_time | { (start_time . naive_local () , TzOffset :: new (* tz , timespans . get (start_idx)) ,) }) } } ; let end = match end_idx { _ if end_idx >= timespans . len () => None , _ => { timespans . local_span (end_idx) . begin . and_then (| end_time | DateTime :: from_timestamp (end_time , 0)) . and_then (| date_time | { tz . from_local_datetime (& date_time . naive_local ()) . single () }) } } ; Some (Self { begin , end }) } }
};
}
