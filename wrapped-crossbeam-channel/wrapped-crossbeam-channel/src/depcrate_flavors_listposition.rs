// Generated macro for Position (struct)
macro_rules! Depcrate_flavors_listPosition {
() => {
// Module: crate::flavors::list
// Provides: {"Position"}
// Dependencies: {}
# [doc = " A position in a channel."] # [derive (Debug)] struct Position < T > { # [doc = " The index in the channel."] index : AtomicUsize , # [doc = " The block in the linked list."] block : AtomicPtr < Block < T > > , }
};
}
