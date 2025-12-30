// Generated macro for tests (module)
macro_rules! Depcrate_styletests {
() => {
// Module: crate::style
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; macro_rules ! skip_windows_ansi_supported { () => { # [cfg (windows)] { if crate :: ansi_support :: supports_ansi () { return ; } } } ; } # [cfg_attr (windows , test)] # [cfg (windows)] fn windows_always_truecolor () { if crate :: ansi_support :: supports_ansi () { assert_eq ! (u16 :: MAX , available_color_count ()) ; } ; } # [test] fn colorterm_overrides_term () { skip_windows_ansi_supported ! () ; temp_env :: with_vars ([("COLORTERM" , Some ("truecolor")) , ("TERM" , Some ("xterm-256color")) ,] , | | { assert_eq ! (u16 :: MAX , available_color_count ()) ; } ,) ; } # [test] fn term_24bits () { skip_windows_ansi_supported ! () ; temp_env :: with_vars ([("COLORTERM" , None) , ("TERM" , Some ("xterm-24bits"))] , | | { assert_eq ! (u16 :: MAX , available_color_count ()) ; } ,) ; } # [test] fn term_256color () { skip_windows_ansi_supported ! () ; temp_env :: with_vars ([("COLORTERM" , None) , ("TERM" , Some ("xterm-256color"))] , | | { assert_eq ! (256u16 , available_color_count ()) ; } ,) ; } # [test] fn default_color_count () { skip_windows_ansi_supported ! () ; temp_env :: with_vars ([("COLORTERM" , None :: < & str >) , ("TERM" , None)] , | | { assert_eq ! (8 , available_color_count ()) ; }) ; } # [test] fn unsupported_term_colorterm_values () { skip_windows_ansi_supported ! () ; temp_env :: with_vars ([("COLORTERM" , Some ("gibberish")) , ("TERM" , Some ("gibberish")) ,] , | | { assert_eq ! (8u16 , available_color_count ()) ; } ,) ; } }
};
}
