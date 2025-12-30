// Generated macro for HandleArc (type)
macro_rules! DepcrateHandleArc {
() => {
// Module: crate
// Provides: {"HandleArc"}
// Dependencies: {}
# [doc = " A thread-local handle to access any object, but thread-safe and independent of the actual type of `OwnShared` or feature toggles in `gix-features`."] pub type HandleArc = Cache < store :: Handle < Arc < Store > > > ;
};
}
