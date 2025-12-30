// Generated macro for impl_734 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_734 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_734"}
// Dependencies: {}
impl UsesLifetimes for Type { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { Type :: Slice (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Array (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Ptr (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Reference (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: BareFn (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Tuple (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Path (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Paren (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Group (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: TraitObject (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: ImplTrait (ref v) => v . uses_lifetimes (options , lifetimes) , Type :: Macro (_) | Type :: Verbatim (_) | Type :: Infer (_) | Type :: Never (_) => { Default :: default () } _ => panic ! ("Unknown syn::Type: {:?}" , self) , } } }
};
}
