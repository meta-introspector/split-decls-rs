// Generated macro for try_opt (macro)
macro_rules! Depcratetry_opt {
() => {
// Module: crate
// Provides: {"try_opt"}
// Dependencies: {}
# [doc = " Workaround because `?` is not (yet) available in const context."] # [macro_export] # [doc (hidden)] macro_rules ! try_opt { ($ e : expr) => { match $ e { Some (v) => v , None => return None , } } ; }
};
}
