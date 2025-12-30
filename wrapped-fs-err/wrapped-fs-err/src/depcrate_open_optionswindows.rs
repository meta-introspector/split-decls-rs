// Generated macro for windows (module)
macro_rules! Depcrate_open_optionswindows {
() => {
// Module: crate::open_options
// Provides: {"windows"}
// Dependencies: {}
# [cfg (windows)] mod windows { use crate :: os :: windows :: fs :: OpenOptionsExt ; use std :: os :: windows :: fs :: OpenOptionsExt as _ ; impl OpenOptionsExt for crate :: OpenOptions { fn access_mode (& mut self , access : u32) -> & mut Self { self . options_mut () . access_mode (access) ; self } fn share_mode (& mut self , val : u32) -> & mut Self { self . options_mut () . share_mode (val) ; self } fn custom_flags (& mut self , flags : u32) -> & mut Self { self . options_mut () . custom_flags (flags) ; self } fn attributes (& mut self , val : u32) -> & mut Self { self . options_mut () . attributes (val) ; self } fn security_qos_flags (& mut self , flags : u32) -> & mut Self { self . options_mut () . security_qos_flags (flags) ; self } } }
};
}
