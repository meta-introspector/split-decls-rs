// Generated macro for version (module)
macro_rules! Depcrateversion {
() => {
// Module: crate
// Provides: {"version"}
// Dependencies: {}
# [doc = " The versions of the server protocol"] pub mod version { pub const NO_VERSION_CHECK_VERSION : u32 = 0 ; pub const VERSION_CHECK_VERSION : u32 = 1 ; pub const ENCODE_CLOSE_SPAN_VERSION : u32 = 2 ; pub const HAS_GLOBAL_SPANS : u32 = 3 ; pub const RUST_ANALYZER_SPAN_SUPPORT : u32 = 4 ; # [doc = " Whether literals encode their kind as an additional u32 field and idents their rawness as a u32 field."] pub const EXTENDED_LEAF_DATA : u32 = 5 ; pub const HASHED_AST_ID : u32 = 6 ; # [doc = " Current API version of the proc-macro protocol."] pub const CURRENT_API_VERSION : u32 = HASHED_AST_ID ; }
};
}
