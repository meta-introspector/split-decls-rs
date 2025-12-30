// Generated macro for HookFunc (type)
macro_rules! Depcrate_configHookFunc {
() => {
// Module: crate::config
// Provides: {"HookFunc"}
// Dependencies: {}
type HookFunc = Box < dyn Fn (& (dyn std :: error :: Error + 'static)) -> Box < dyn eyre :: EyreHandler > + Send + Sync + 'static , > ;
};
}
