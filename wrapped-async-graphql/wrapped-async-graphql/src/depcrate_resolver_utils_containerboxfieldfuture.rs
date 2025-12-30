// Generated macro for BoxFieldFuture (type)
macro_rules! Depcrate_resolver_utils_containerBoxFieldFuture {
() => {
// Module: crate::resolver_utils::container
// Provides: {"BoxFieldFuture"}
// Dependencies: {}
type BoxFieldFuture < 'a > = Pin < Box < dyn Future < Output = ServerResult < (Name , Value) > > + 'a + Send > > ;
};
}
