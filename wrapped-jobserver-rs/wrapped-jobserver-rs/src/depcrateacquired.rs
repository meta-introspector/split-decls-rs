// Generated macro for Acquired (struct)
macro_rules! DepcrateAcquired {
() => {
// Module: crate
// Provides: {"Acquired"}
// Dependencies: {}
# [doc = " An acquired token from a jobserver."] # [doc = ""] # [doc = " This token will be released back to the jobserver when it is dropped and"] # [doc = " otherwise represents the ability to spawn off another thread of work."] # [derive (Debug)] pub struct Acquired { client : Arc < imp :: Client > , data : imp :: Acquired , disabled : bool , }
};
}
