// Generated macro for format_params (macro)
macro_rules! Depcrateformat_params {
() => {
// Module: crate
// Provides: {"format_params"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! format_params { ($ single : ident) => { "{}" } ; ($ first : ident , $ ($ rest : ident) ,+) => { concat ! ("{}" , ", " , $ crate :: format_params ! ($ ($ rest) ,+)) } ; }
};
}
