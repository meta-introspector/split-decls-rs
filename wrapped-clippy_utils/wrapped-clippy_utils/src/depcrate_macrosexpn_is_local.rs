// Generated macro for expn_is_local (function)
macro_rules! Depcrate_macrosexpn_is_local {
() => {
// Module: crate::macros
// Provides: {"expn_is_local"}
// Dependencies: {}
# [doc = " Checks whether the expansion is the root expansion or a locally defined macro"] pub fn expn_is_local (expn : ExpnId) -> bool { if expn == ExpnId :: root () { return true ; } let data = expn . expn_data () ; let backtrace = expn_backtrace (data . call_site) ; std :: iter :: once ((expn , data)) . chain (backtrace) . find_map (| (_ , data) | data . macro_def_id) . is_none_or (DefId :: is_local) }
};
}
