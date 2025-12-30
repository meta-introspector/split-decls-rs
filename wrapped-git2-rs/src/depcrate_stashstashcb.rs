// Generated macro for StashCb (type)
macro_rules! Depcrate_stashStashCb {
() => {
// Module: crate::stash
// Provides: {"StashCb"}
// Dependencies: {}
# [doc = " This is a callback function you can provide to iterate over all the"] # [doc = " stashed states that will be invoked per entry."] pub type StashCb < 'a > = dyn FnMut (usize , & str , & Oid) -> bool + 'a ;
};
}
