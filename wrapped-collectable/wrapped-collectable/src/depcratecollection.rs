// Generated macro for Collection (trait)
macro_rules! DepcrateCollection {
() => {
// Module: crate
// Provides: {"Collection"}
// Dependencies: {}
# [doc = " Collection types implement all of the traits in this crate."] pub trait Collection < T > : AsRef < [T] > + AsMut < [T] > + Default + Length + Truncate + TryExtend < T > + TryPush < T > { }
};
}
