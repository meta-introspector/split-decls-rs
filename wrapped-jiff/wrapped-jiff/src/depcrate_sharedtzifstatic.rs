// Generated macro for TzifStatic (type)
macro_rules! Depcrate_sharedTzifStatic {
() => {
// Module: crate::shared
// Provides: {"TzifStatic"}
// Dependencies: {}
# [doc = " An alias for TZif data whose backing storage has a `'static` lifetime."] pub type TzifStatic = Tzif < & 'static str , & 'static str , & 'static [TzifLocalTimeType] , & 'static [i64] , & 'static [TzifDateTime] , & 'static [TzifDateTime] , & 'static [TzifTransitionInfo] , > ;
};
}
