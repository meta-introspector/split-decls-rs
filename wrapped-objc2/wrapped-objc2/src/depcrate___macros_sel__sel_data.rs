// Generated macro for __sel_data (macro)
macro_rules! Depcrate___macros_sel__sel_data {
() => {
// Module: crate::__macros::sel
// Provides: {"__sel_data"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __sel_data { ($ first : ident $ (: $ ($ ($ rest : ident) ? :) *) ?) => { $ crate :: __macros :: concat ! ($ crate :: __macros :: stringify ! ($ first) , $ (':' , $ ($ ($ crate :: __macros :: stringify ! ($ rest) ,) ? ':' ,) *) ?) } ; }
};
}
