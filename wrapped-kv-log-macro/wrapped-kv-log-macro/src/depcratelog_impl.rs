// Generated macro for log_impl (macro)
macro_rules! Depcratelog_impl {
() => {
// Module: crate
// Provides: {"log_impl"}
// Dependencies: {}
# [macro_export (local_inner_macros)] # [doc (hidden)] macro_rules ! log_impl { (target : $ target : expr , $ lvl : expr , ($ ($ arg : expr) ,*)) => { { let lvl = $ lvl ; if lvl <= $ crate :: STATIC_MAX_LEVEL && lvl <= $ crate :: max_level () { $ crate :: __private_api_log (__log_format_args ! ($ ($ arg) ,*) , lvl , & ($ target , __log_module_path ! () , __log_file ! () , __log_line ! ()) , None ,) ; } } } ; (target : $ target : expr , $ lvl : expr , ($ ($ arg : expr) ,*) { $ ($ key : ident : $ value : expr) ,* }) => { { if $ lvl <= $ crate :: STATIC_MAX_LEVEL && $ lvl <= $ crate :: max_level () { $ crate :: __private_api_log (__log_format_args ! ($ ($ arg) ,*) , $ lvl , & (__log_module_path ! () , __log_module_path ! () , __log_file ! () , __log_line ! ()) , Some (& [$ ((__log_stringify ! ($ key) , &$ value)) ,*])) ; } } } ; (target : $ target : expr , $ lvl : expr , ($ ($ e : expr) ,*) { $ ($ key : ident : $ value : expr ,) * }) => { $ crate :: log_impl ! (target : $ target , $ lvl , ($ ($ e) ,*) { $ ($ key : $ value) ,* }) ; } ; (target : $ target : expr , $ lvl : expr , ($ ($ e : expr) ,*) $ arg : expr) => { $ crate :: log_impl ! (target : $ target , $ lvl , ($ ($ e ,) * $ arg)) ; } ; (target : $ target : expr , $ lvl : expr , ($ ($ e : expr) ,*) $ arg : expr , $ ($ rest : tt) *) => { $ crate :: log_impl ! (target : $ target , $ lvl , ($ ($ e ,) * $ arg) $ ($ rest) *) ; } ; }
};
}
