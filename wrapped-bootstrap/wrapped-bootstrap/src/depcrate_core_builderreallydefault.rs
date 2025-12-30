// Generated macro for ReallyDefault (enum)
macro_rules! Depcrate_core_builderReallyDefault {
() => {
// Module: crate::core::builder
// Provides: {"ReallyDefault"}
// Dependencies: {}
enum ReallyDefault < 'a > { Bool (bool) , Lazy (LazyLock < bool , Box < dyn Fn () -> bool + 'a > >) , }
};
}
