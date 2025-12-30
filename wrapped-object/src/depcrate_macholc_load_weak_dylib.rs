// Generated macro for LC_LOAD_WEAK_DYLIB (const)
macro_rules! Depcrate_machoLC_LOAD_WEAK_DYLIB {
() => {
// Module: crate::macho
// Provides: {"LC_LOAD_WEAK_DYLIB"}
// Dependencies: {}
# [doc = " load a dynamically linked shared library that is allowed to be missing"] # [doc = " (all symbols are weak imported)."] pub const LC_LOAD_WEAK_DYLIB : u32 = 0x18 | LC_REQ_DYLD ;
};
}
