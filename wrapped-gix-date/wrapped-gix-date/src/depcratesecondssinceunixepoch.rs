// Generated macro for SecondsSinceUnixEpoch (type)
macro_rules! DepcrateSecondsSinceUnixEpoch {
() => {
// Module: crate
// Provides: {"SecondsSinceUnixEpoch"}
// Dependencies: {}
# [doc = " The number of seconds since unix epoch."] # [doc = ""] # [doc = " Note that negative dates represent times before the unix epoch."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " `git` only supports dates *from* the UNIX epoch, whereas we chose to be more flexible at the expense of stopping time"] # [doc = " a few million years before the heat-death of the universe."] pub type SecondsSinceUnixEpoch = i64 ;
};
}
