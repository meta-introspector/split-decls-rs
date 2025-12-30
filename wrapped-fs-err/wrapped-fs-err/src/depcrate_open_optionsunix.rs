// Generated macro for unix (module)
macro_rules! Depcrate_open_optionsunix {
() => {
// Module: crate::open_options
// Provides: {"unix"}
// Dependencies: {}
# [cfg (unix)] mod unix { use crate :: os :: unix :: fs :: OpenOptionsExt ; use std :: os :: unix :: fs :: OpenOptionsExt as _ ; impl OpenOptionsExt for crate :: OpenOptions { fn mode (& mut self , mode : u32) -> & mut Self { self . options_mut () . mode (mode) ; self } fn custom_flags (& mut self , flags : i32) -> & mut Self { self . options_mut () . custom_flags (flags) ; self } } }
};
}
