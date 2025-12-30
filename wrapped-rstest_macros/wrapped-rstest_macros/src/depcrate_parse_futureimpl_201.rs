// Generated macro for impl_201 (impl)
macro_rules! Depcrate_parse_futureimpl_201 {
() => {
// Module: crate::parse::future
// Provides: {"impl_201"}
// Dependencies: {}
impl MaybeFutureImplType for FnArg { fn as_future_impl_type (& self) -> Option < & Type > { match self { FnArg :: Typed (PatType { ty , .. }) if can_impl_future (ty . as_ref ()) => Some (ty . as_ref ()) , _ => None , } } fn as_mut_future_impl_type (& mut self) -> Option < & mut Type > { match self { FnArg :: Typed (PatType { ty , .. }) if can_impl_future (ty . as_ref ()) => Some (ty . as_mut ()) , _ => None , } } }
};
}
