// Generated macro for expand (function)
macro_rules! Depcrate_cfg_evalexpand {
() => {
// Module: crate::cfg_eval
// Provides: {"expand"}
// Dependencies: {}
pub (crate) fn expand (ecx : & mut ExtCtxt < '_ > , _span : Span , meta_item : & ast :: MetaItem , annotatable : Annotatable ,) -> Vec < Annotatable > { check_builtin_macro_attribute (ecx , meta_item , sym :: cfg_eval) ; warn_on_duplicate_attribute (ecx , & annotatable , sym :: cfg_eval) ; vec ! [cfg_eval (ecx . sess , ecx . ecfg . features , annotatable , ecx . current_expansion . lint_node_id)] }
};
}
