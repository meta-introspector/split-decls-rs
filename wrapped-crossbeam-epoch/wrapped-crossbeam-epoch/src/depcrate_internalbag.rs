// Generated macro for Bag (struct)
macro_rules! Depcrate_internalBag {
() => {
// Module: crate::internal
// Provides: {"Bag"}
// Dependencies: {}
# [doc = " A bag of deferred functions."] pub (crate) struct Bag { # [doc = " Stashed objects."] deferreds : [Deferred ; MAX_OBJECTS] , len : usize , }
};
}
