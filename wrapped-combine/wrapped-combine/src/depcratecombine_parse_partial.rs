// Generated macro for combine_parse_partial (macro)
macro_rules! Depcratecombine_parse_partial {
() => {
// Module: crate
// Provides: {"combine_parse_partial"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! combine_parse_partial { ((()) $ mode : ident $ input : ident $ state : ident $ parser : block) => { { let _ = $ state ; let mut state = Default :: default () ; let state = & mut state ; $ parser . parse_mode ($ mode , $ input , state) } } ; (($ ignored : ty) $ mode : ident $ input : ident $ state : ident $ parser : block) => { $ parser . parse_mode ($ mode , $ input , $ state) } ; }
};
}
