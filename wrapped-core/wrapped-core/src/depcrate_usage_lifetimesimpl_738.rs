// Generated macro for impl_738 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_738 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_738"}
// Dependencies: {}
impl UsesLifetimes for syn :: PathArguments { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { syn :: PathArguments :: None => Default :: default () , syn :: PathArguments :: AngleBracketed (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: PathArguments :: Parenthesized (ref v) => v . uses_lifetimes (options , lifetimes) , } } }
};
}
