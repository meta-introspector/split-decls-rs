// Generated macro for validate_input (function)
macro_rules! Depcrate_cfg_accessiblevalidate_input {
() => {
// Module: crate::cfg_accessible
// Provides: {"validate_input"}
// Dependencies: {}
fn validate_input < 'a > (ecx : & ExtCtxt < '_ > , mi : & 'a ast :: MetaItem) -> Option < & 'a ast :: Path > { use errors :: CfgAccessibleInvalid :: * ; match mi . meta_item_list () { None => { } Some ([]) => { ecx . dcx () . emit_err (UnspecifiedPath (mi . span)) ; } Some ([_ , .. , l]) => { ecx . dcx () . emit_err (MultiplePaths (l . span ())) ; } Some ([nmi]) => match nmi . meta_item () { None => { ecx . dcx () . emit_err (LiteralPath (nmi . span ())) ; } Some (mi) => { if ! mi . is_word () { ecx . dcx () . emit_err (HasArguments (mi . span)) ; } return Some (& mi . path) ; } } , } None }
};
}
