// Generated macro for ErrorHook (type)
macro_rules! DepcrateErrorHook {
() => {
// Module: crate
// Provides: {"ErrorHook"}
// Dependencies: {}
type ErrorHook = Box < dyn Fn (& (dyn StdError + 'static)) -> Box < dyn EyreHandler > + Sync + Send + 'static > ;
};
}
