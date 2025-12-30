// Generated macro for OpenOptions (struct)
macro_rules! Depcrate_open_optionsOpenOptions {
() => {
// Module: crate::open_options
// Provides: {"OpenOptions"}
// Dependencies: {}
# [doc = " Options and flags used to configure how a registry key is opened."] # [derive (Debug)] pub struct OpenOptions < 'a > { parent : & 'a Key , access : u32 , create : bool , transaction : Option < & 'a Transaction > , options : u32 , }
};
}
