// Generated macro for BoxResolverFn (type)
macro_rules! Depcrate_dynamic_fieldBoxResolverFn {
() => {
// Module: crate::dynamic::field
// Provides: {"BoxResolverFn"}
// Dependencies: {}
pub (crate) type BoxResolverFn = Box < (dyn for < 'a > Fn (ResolverContext < 'a >) -> FieldFuture < 'a > + Send + Sync) > ;
};
}
