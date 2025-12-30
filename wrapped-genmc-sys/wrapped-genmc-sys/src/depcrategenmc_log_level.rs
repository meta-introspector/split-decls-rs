// Generated macro for GENMC_LOG_LEVEL (static)
macro_rules! DepcrateGENMC_LOG_LEVEL {
() => {
// Module: crate
// Provides: {"GENMC_LOG_LEVEL"}
// Dependencies: {}
# [doc = " Changing GenMC's log level is not thread safe, so we limit it to only be set once to prevent any data races."] # [doc = " This value will be initialized when the first `MiriGenmcShim` is created."] static GENMC_LOG_LEVEL : OnceLock < LogLevel > = OnceLock :: new () ;
};
}
