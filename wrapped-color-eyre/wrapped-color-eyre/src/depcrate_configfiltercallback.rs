// Generated macro for FilterCallback (type)
macro_rules! Depcrate_configFilterCallback {
() => {
// Module: crate::config
// Provides: {"FilterCallback"}
// Dependencies: {}
# [doc = " Callback for filtering a vector of `Frame`s"] pub type FilterCallback = dyn Fn (& mut Vec < & Frame >) + Send + Sync + 'static ;
};
}
