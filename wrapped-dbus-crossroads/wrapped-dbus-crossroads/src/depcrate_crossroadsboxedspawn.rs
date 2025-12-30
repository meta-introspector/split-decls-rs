// Generated macro for BoxedSpawn (type)
macro_rules! Depcrate_crossroadsBoxedSpawn {
() => {
// Module: crate::crossroads
// Provides: {"BoxedSpawn"}
// Dependencies: {}
pub type BoxedSpawn = Box < dyn Fn (Pin < Box < dyn Future < Output = () > + Send + 'static > >) + Send + 'static > ;
};
}
