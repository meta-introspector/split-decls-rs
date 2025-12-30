// Generated macro for impl_733 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_733 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_733"}
// Dependencies: {}
impl UsesLifetimes for syn :: Data { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { syn :: Data :: Struct (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: Data :: Enum (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: Data :: Union (ref v) => v . uses_lifetimes (options , lifetimes) , } } }
};
}
