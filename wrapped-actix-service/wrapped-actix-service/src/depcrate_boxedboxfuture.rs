// Generated macro for BoxFuture (type)
macro_rules! Depcrate_boxedBoxFuture {
() => {
// Module: crate::boxed
// Provides: {"BoxFuture"}
// Dependencies: {}
# [doc = " A boxed future with no send bound or lifetime parameters."] pub type BoxFuture < T > = Pin < Box < dyn Future < Output = T > > > ;
};
}
