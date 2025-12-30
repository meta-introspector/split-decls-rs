// Generated macro for Config (struct)
macro_rules! Depcrate_jsonConfig {
() => {
// Module: crate::json
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration for the JSON printer."] # [doc = ""] # [doc = " This is manipulated by the JSONBuilder and then referenced by the actual"] # [doc = " implementation. Once a printer is build, the configuration is frozen and"] # [doc = " cannot changed."] # [derive (Debug , Clone)] struct Config { pretty : bool , always_begin_end : bool , replacement : Arc < Option < Vec < u8 > > > , }
};
}
