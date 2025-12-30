// Generated macro for MRAwait (struct)
macro_rules! Depcrate_nonblockMRAwait {
() => {
// Module: crate::nonblock
// Provides: {"MRAwait"}
// Dependencies: {}
struct MRAwait { mrouter : MROuter , token : Result < Token , () > , timeout : Instant , timeoutfn : Option < TimeoutMakerCb > }
};
}
