// Generated macro for Configuration (struct)
macro_rules! DepcrateConfiguration {
() => {
// Module: crate
// Provides: {"Configuration"}
// Dependencies: {}
# [doc = " Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`] instead."] # [deprecated (note = "Use `ThreadPoolBuilder`")] # [derive (Default)] pub struct Configuration { builder : ThreadPoolBuilder , }
};
}
