// Generated macro for __console_inner (macro)
macro_rules! Depcrate_console_dbg__console_inner {
() => {
// Module: crate::console_dbg
// Provides: {"__console_inner"}
// Dependencies: {}
# [doc = " This is an implementation detail and *should not* be called directly!"] # [doc (hidden)] # [macro_export] macro_rules ! __console_inner { ($ js_value : ident $ val : expr) => { { $ crate :: log ! (:: std :: format ! ("%c[{}:{}] " , :: std :: file ! () , :: std :: line ! ()) , "font-weight: bold" , :: std :: format ! ("{} = " , :: std :: stringify ! ($ val)) , &$ js_value) ; $ js_value } } ; }
};
}
