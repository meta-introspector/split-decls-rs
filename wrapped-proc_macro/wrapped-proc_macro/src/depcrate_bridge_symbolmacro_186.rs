// Generated macro for macro_186 (macro)
macro_rules! Depcrate_bridge_symbolmacro_186 {
() => {
// Module: crate::bridge::symbol
// Provides: {"macro_186"}
// Dependencies: {}
thread_local ! { static INTERNER : RefCell < Interner > = RefCell :: new (Interner { arena : arena :: Arena :: new () , names : fxhash :: FxHashMap :: default () , strings : Vec :: new () , sym_base : NonZero :: new (1) . unwrap () , }) ; }
};
}
