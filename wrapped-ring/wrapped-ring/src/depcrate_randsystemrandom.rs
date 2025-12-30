// Generated macro for SystemRandom (struct)
macro_rules! Depcrate_randSystemRandom {
() => {
// Module: crate::rand
// Provides: {"SystemRandom"}
// Dependencies: {}
# [doc = " A secure random number generator where the random values come directly"] # [doc = " from the operating system."] # [doc = ""] # [doc = " \"Directly from the operating system\" here presently means \"whatever the"] # [doc = " `getrandom` crate does\" but that may change in the future. That roughly"] # [doc = " means calling libc's `getrandom` function or whatever is analogous to that;"] # [doc = " see the `getrandom` crate's documentation for more info."] # [doc = ""] # [doc = " A single `SystemRandom` may be shared across multiple threads safely."] # [doc = ""] # [doc = " `new()` is guaranteed to always succeed and to have low latency; it won't"] # [doc = " try to open or read from a file or do similar things. The first call to"] # [doc = " `fill()` may block a substantial amount of time since any and all"] # [doc = " initialization is deferred to it. Therefore, it may be a good idea to call"] # [doc = " `fill()` once at a non-latency-sensitive time to minimize latency for"] # [doc = " future calls."] # [derive (Clone , Debug)] pub struct SystemRandom (()) ;
};
}
