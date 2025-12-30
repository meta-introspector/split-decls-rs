// Generated macro for is_reserved_version (function)
macro_rules! Depcrateis_reserved_version {
() => {
// Module: crate
// Provides: {"is_reserved_version"}
// Dependencies: {}
fn is_reserved_version (version : u32) -> bool { version & RESERVED_VERSION_MASK == version }
};
}
