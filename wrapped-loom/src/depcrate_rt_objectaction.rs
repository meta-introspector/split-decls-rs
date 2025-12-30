// Generated macro for Action (enum)
macro_rules! Depcrate_rt_objectAction {
() => {
// Module: crate::rt::object
// Provides: {"Action"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq)] pub (super) enum Action { # [doc = " Action on an Arc object"] Arc (rt :: arc :: Action) , # [doc = " Action on an atomic object"] Atomic (rt :: atomic :: Action) , # [doc = " Action on a channel"] Channel (rt :: mpsc :: Action) , # [doc = " Action on a RwLock"] RwLock (rt :: rwlock :: Action) , # [doc = " Generic action with no specialized dependencies on access."] Opaque , }
};
}
