// Generated macro for log (macro)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
# [doc = " The standard logging macro."] # [doc = ""] # [doc = " ```"] # [doc = " use kv_log_macro::info;"] # [doc = ""] # [doc = " info!(\"hello\");"] # [doc = " info!(\"hello\",);"] # [doc = " info!(\"hello {}\", \"cats\");"] # [doc = " info!(\"hello {}\", \"cats\",);"] # [doc = " info!(\"hello {}\", \"cats\", {"] # [doc = "     cat_1: \"chashu\","] # [doc = "     cat_2: \"nori\","] # [doc = " });"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! log { (target : $ target : expr , $ lvl : expr , $ e : expr) => { $ crate :: log_impl ! (target : $ target , $ lvl , ($ e)) ; } ; (target : $ target : expr , $ lvl : expr , $ e : expr , $ ($ rest : tt) *) => { $ crate :: log_impl ! (target : $ target , $ lvl , ($ e) $ ($ rest) *) ; } ; ($ lvl : expr , $ ($ arg : tt) +) => ($ crate :: log ! (target : __log_module_path ! () , $ lvl , $ ($ arg) +)) }
};
}
