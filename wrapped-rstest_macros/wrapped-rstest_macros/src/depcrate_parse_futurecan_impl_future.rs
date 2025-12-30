// Generated macro for can_impl_future (function)
macro_rules! Depcrate_parse_futurecan_impl_future {
() => {
// Module: crate::parse::future
// Provides: {"can_impl_future"}
// Dependencies: {}
fn can_impl_future (ty : & Type) -> bool { use Type :: * ; ! matches ! (ty , Group (_) | ImplTrait (_) | Infer (_) | Macro (_) | Never (_) | Slice (_) | TraitObject (_) | Verbatim (_)) }
};
}
