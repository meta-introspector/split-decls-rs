// Generated macro for ExecuteFutFactory (type)
macro_rules! Depcrate_extensionsExecuteFutFactory {
() => {
// Module: crate::extensions
// Provides: {"ExecuteFutFactory"}
// Dependencies: {}
type ExecuteFutFactory < 'a > = Box < dyn FnOnce (Option < Data >) -> BoxFuture < 'a , Response > + Send + 'a > ;
};
}
