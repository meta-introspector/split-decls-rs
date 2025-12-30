// Generated macro for __log (macro)
macro_rules! Depcrate_macros__log {
() => {
// Module: crate::macros
// Provides: {"__log"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __log { (logger : $ logger : expr , target : $ target : expr , $ lvl : expr , $ ($ key : tt $ (:$ capture : tt) ? $ (= $ value : expr) ?) ,+; $ ($ arg : tt) +) => ({ let lvl = $ lvl ; if lvl <= $ crate :: STATIC_MAX_LEVEL && lvl <= $ crate :: max_level () { $ crate :: __private_api :: log ($ logger , $ crate :: __private_api :: format_args ! ($ ($ arg) +) , lvl , & ($ target , $ crate :: __private_api :: module_path ! () , $ crate :: __private_api :: loc ()) , & [$ (($ crate :: __log_key ! ($ key) , $ crate :: __log_value ! ($ key $ (:$ capture) * = $ ($ value) *))) ,+] as & [_] ,) ; } }) ; (logger : $ logger : expr , target : $ target : expr , $ lvl : expr , $ ($ arg : tt) +) => ({ let lvl = $ lvl ; if lvl <= $ crate :: STATIC_MAX_LEVEL && lvl <= $ crate :: max_level () { $ crate :: __private_api :: log ($ logger , $ crate :: __private_api :: format_args ! ($ ($ arg) +) , lvl , & ($ target , $ crate :: __private_api :: module_path ! () , $ crate :: __private_api :: loc ()) , () ,) ; } }) ; }
};
}
