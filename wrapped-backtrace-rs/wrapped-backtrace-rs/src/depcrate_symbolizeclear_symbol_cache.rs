// Generated macro for clear_symbol_cache (function)
macro_rules! Depcrate_symbolizeclear_symbol_cache {
() => {
// Module: crate::symbolize
// Provides: {"clear_symbol_cache"}
// Dependencies: {}
# [doc = " Attempt to reclaim that cached memory used to symbolicate addresses."] # [doc = ""] # [doc = " This method will attempt to release any global data structures that have"] # [doc = " otherwise been cached globally or in the thread which typically represent"] # [doc = " parsed DWARF information or similar."] # [doc = ""] # [doc = " # Caveats"] # [doc = ""] # [doc = " While this function is always available it doesn't actually do anything on"] # [doc = " most implementations. Libraries like dbghelp or libbacktrace do not provide"] # [doc = " facilities to deallocate state and manage the allocated memory. For now the"] # [doc = " `std` feature of this crate is the only feature where this"] # [doc = " function has any effect."] # [cfg (feature = "std")] pub fn clear_symbol_cache () { let _guard = crate :: lock :: lock () ; unsafe { imp :: clear_symbol_cache () ; } }
};
}
