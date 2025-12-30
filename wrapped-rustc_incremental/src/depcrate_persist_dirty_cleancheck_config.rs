// Generated macro for check_config (function)
macro_rules! Depcrate_persist_dirty_cleancheck_config {
() => {
// Module: crate::persist::dirty_clean
// Provides: {"check_config"}
// Dependencies: {}
# [doc = " Given a `#[rustc_clean]` attribute, scan for a `cfg=\"foo\"` attribute and check whether we have"] # [doc = " a cfg flag called `foo`."] fn check_config (tcx : TyCtxt < '_ > , attr : & Attribute) -> bool { debug ! ("check_config(attr={:?})" , attr) ; let config = & tcx . sess . psess . config ; debug ! ("check_config: config={:?}" , config) ; let mut cfg = None ; for item in attr . meta_item_list () . unwrap_or_else (ThinVec :: new) { if item . has_name (CFG) { let value = expect_associated_value (tcx , & item) ; debug ! ("check_config: searching for cfg {:?}" , value) ; cfg = Some (config . contains (& (value , None))) ; } else if ! (item . has_name (EXCEPT) || item . has_name (LOADED_FROM_DISK)) { tcx . dcx () . emit_err (errors :: UnknownRustcCleanArgument { span : item . span () }) ; } } match cfg { None => tcx . dcx () . emit_fatal (errors :: NoCfg { span : attr . span () }) , Some (c) => c , } }
};
}
