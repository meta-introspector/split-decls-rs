// Generated macro for Fountain (struct)
macro_rules! Depcrate_utilsFountain {
() => {
// Module: crate::utils
// Provides: {"Fountain"}
// Dependencies: {}
# [doc = " A simple producer of infinite stream of bytes, useful in tests."] # [doc = ""] # [doc = " Will repeat `chunk` field indefinitely."] pub struct Fountain < 'a > { # [doc = " That piece of data repeated infinitely..."] pub chunk : & 'a [u8] , # [doc = " Part of `chunk` that was consumed by BufRead impl"] pub consumed : usize , # [doc = " The overall count of read bytes"] pub overall_read : u64 , }
};
}
