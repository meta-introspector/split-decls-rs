// Generated macro for try_begin_demangle_as_inner (macro)
macro_rules! Depcrate_asttry_begin_demangle_as_inner {
() => {
// Module: crate::ast
// Provides: {"try_begin_demangle_as_inner"}
// Dependencies: {}
# [doc = " Automatically log start and end demangling in an s-expression format, when"] # [doc = " the `logging` feature is enabled."] macro_rules ! try_begin_demangle_as_inner { ($ production : expr , $ ctx : expr , $ scope : expr) => { { let _log = AutoLogDemangle :: new ($ production , $ ctx , $ scope , true) ; & mut AutoParseDemangle :: new ($ ctx) ? } } ; }
};
}
