// Generated macro for BoxFuture (type)
macro_rules! DepcrateBoxFuture {
() => {
// Module: crate
// Provides: {"BoxFuture"}
// Dependencies: {}
type BoxFuture < T > = std :: pin :: Pin < Box < dyn std :: future :: Future < Output = T > > > ;
};
}
