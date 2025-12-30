// Generated macro for BoxedFut (type)
macro_rules! Depcrate_clientBoxedFut {
() => {
// Module: crate::client
// Provides: {"BoxedFut"}
// Dependencies: {}
type BoxedFut < T > = Pin < Box < dyn Future < Output = Result < MaybeHttpsStream < T > , BoxError > > + Send > > ;
};
}
