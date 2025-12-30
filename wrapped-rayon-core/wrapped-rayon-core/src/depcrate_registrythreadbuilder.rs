// Generated macro for ThreadBuilder (struct)
macro_rules! Depcrate_registryThreadBuilder {
() => {
// Module: crate::registry
// Provides: {"ThreadBuilder"}
// Dependencies: {}
# [doc = " Thread builder used for customization via [`ThreadPoolBuilder::spawn_handler()`]."] pub struct ThreadBuilder { name : Option < String > , stack_size : Option < usize > , worker : Worker < JobRef > , stealer : Stealer < JobRef > , registry : Arc < Registry > , index : usize , }
};
}
