// Generated macro for vmctx_addr (function)
macro_rules! Depcrate_legalizer_globalvaluevmctx_addr {
() => {
// Module: crate::legalizer::globalvalue
// Provides: {"vmctx_addr"}
// Dependencies: {}
# [doc = " Expand a `global_value` instruction for a vmctx global."] fn vmctx_addr (global_value : ir :: GlobalValue , inst : ir :: Inst , func : & mut ir :: Function) { let vmctx = func . special_param (ir :: ArgumentPurpose :: VMContext) . expect ("Missing vmctx parameter") ; let result = func . dfg . first_result (inst) ; func . dfg . clear_results (inst) ; func . dfg . change_to_alias (result , vmctx) ; func . layout . remove_inst (inst) ; if let Some (fact) = & func . global_value_facts [global_value] { if func . dfg . facts [vmctx] . is_none () { let fact = fact . clone () ; func . dfg . facts [vmctx] = Some (fact) ; } } }
};
}
