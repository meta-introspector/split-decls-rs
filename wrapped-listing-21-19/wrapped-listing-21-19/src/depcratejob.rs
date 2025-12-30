// Generated macro for Job (type)
macro_rules! DepcrateJob {
() => {
// Module: crate
// Provides: {"Job"}
// Dependencies: {}
type Job = Box < dyn FnOnce () + Send + 'static > ;
};
}
