// Generated macro for RustfmtConfig (enum)
macro_rules! Depcrate_configRustfmtConfig {
() => {
// Module: crate::config
// Provides: {"RustfmtConfig"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum RustfmtConfig { Rustfmt { extra_args : Vec < String > , enable_range_formatting : bool } , CustomCommand { command : String , args : Vec < String > } , }
};
}
