// Generated macro for try_begin_demangle (macro)
macro_rules! Depcrate_asttry_begin_demangle {
() => {
// Module: crate::ast
// Provides: {"try_begin_demangle"}
// Dependencies: {}
# [doc = " Automatically log start and end demangling in an s-expression format, when"] # [doc = " the `logging` feature is enabled."] macro_rules ! try_begin_demangle { ($ production : expr , $ ctx : expr , $ scope : expr) => { { let _log = AutoLogDemangle :: new ($ production , $ ctx , $ scope , false) ; & mut AutoParseDemangle :: new ($ ctx) ? } } ; }
};
}
