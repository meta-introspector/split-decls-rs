// Generated macro for Sleepers (struct)
macro_rules! DepcrateSleepers {
() => {
// Module: crate
// Provides: {"Sleepers"}
// Dependencies: {}
# [doc = " A list of sleeping tickers."] struct Sleepers { # [doc = " Number of sleeping tickers (both notified and unnotified)."] count : usize , # [doc = " IDs and wakers of sleeping unnotified tickers."] # [doc = ""] # [doc = " A sleeping ticker is notified when its waker is missing from this list."] wakers : Vec < (usize , Waker) > , # [doc = " Reclaimed IDs."] free_ids : Vec < usize > , }
};
}
