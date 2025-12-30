// Generated macro for Flags (struct)
macro_rules! Depcrate_parserFlags {
() => {
// Module: crate::parser
// Provides: {"Flags"}
// Dependencies: {}
# [doc = " Flag state used in the parser."] # [derive (Clone , Copy , Debug)] pub struct Flags { # [doc = " i"] pub casei : bool , # [doc = " m"] pub multi : bool , # [doc = " s"] pub dotnl : bool , # [doc = " U"] pub swap_greed : bool , # [doc = " x"] pub ignore_space : bool , # [doc = " u"] pub unicode : bool , # [doc = " Not actually a flag, but when disabled, every regex that may not match"] # [doc = " UTF-8 exclusively will cause the parser to return an error."] pub allow_bytes : bool , }
};
}
