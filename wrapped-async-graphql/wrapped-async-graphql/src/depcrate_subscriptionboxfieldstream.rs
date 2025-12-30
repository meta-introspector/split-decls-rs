// Generated macro for BoxFieldStream (type)
macro_rules! Depcrate_subscriptionBoxFieldStream {
() => {
// Module: crate::subscription
// Provides: {"BoxFieldStream"}
// Dependencies: {}
pub (crate) type BoxFieldStream < 'a > = Pin < Box < dyn Stream < Item = Response > + 'a + Send > > ;
};
}
