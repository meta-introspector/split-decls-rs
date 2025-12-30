// Generated macro for INTERNER (static)
macro_rules! Depcrate_utils_cacheINTERNER {
() => {
// Module: crate::utils::cache
// Provides: {"INTERNER"}
// Dependencies: {}
# [doc = " A global instance of `Interner` that caches common interned values."] pub static INTERNER : LazyLock < Interner > = LazyLock :: new (Interner :: default) ;
};
}
