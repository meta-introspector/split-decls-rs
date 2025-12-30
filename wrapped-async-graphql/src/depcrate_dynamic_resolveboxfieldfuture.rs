// Generated macro for BoxFieldFuture (type)
macro_rules! Depcrate_dynamic_resolveBoxFieldFuture {
() => {
// Module: crate::dynamic::resolve
// Provides: {"BoxFieldFuture"}
// Dependencies: {}
type BoxFieldFuture < 'a > = Pin < Box < dyn Future < Output = ServerResult < (Name , Value) > > + 'a + Send > > ;
};
}
