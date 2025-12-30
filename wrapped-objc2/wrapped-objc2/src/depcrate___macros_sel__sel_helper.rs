// Generated macro for __sel_helper (macro)
macro_rules! Depcrate___macros_sel__sel_helper {
() => {
// Module: crate::__macros::sel
// Provides: {"__sel_helper"}
// Dependencies: {}
# [doc = " Handle selectors with internal colons."] # [doc = ""] # [doc = " Required since `::` is a different token than `:`."] # [doc (hidden)] # [macro_export] macro_rules ! __sel_helper { { ($ ($ parsed_sel : tt) *) } => { $ crate :: __sel_data ! ($ ($ parsed_sel) *) } ; { () $ ident : ident } => { $ crate :: __sel_helper ! { ($ ident) } } ; { ($ ($ parsed_sel : tt) *) $ ($ ident : ident) ? : $ ($ rest : tt) * } => { $ crate :: __sel_helper ! { ($ ($ parsed_sel) * $ ($ ident) ? :) $ ($ rest) * } } ; { ($ ($ parsed_sel : tt) *) $ ($ ident : ident) ? :: $ ($ rest : tt) * } => { $ crate :: __sel_helper ! { ($ ($ parsed_sel) * $ ($ ident) ? : :) $ ($ rest) * } } ; }
};
}
