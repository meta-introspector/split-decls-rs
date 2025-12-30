// Generated macro for split_borrow (macro)
macro_rules! Depcrate_macrossplit_borrow {
() => {
// Module: crate::macros
// Provides: {"split_borrow"}
// Dependencies: {}
# [doc = " Helper macro for splitting a partial reference."] # [doc = ""] # [doc = " The statement `split_borrow!(target, rest = &(...) expr)` where `...` is a list of parts (same"] # [doc = " syntax as used for [`partial`]) splits the partial reference returned by expr and binds the listed"] # [doc = " parts to the identifier `target` and the remaining parts to the identifier `rest`. This is done"] # [doc = " using [`PartialRef`]'s split_borrow method."] # [doc = ""] # [doc = " As shorter version is available as `split_borrow!(target = &(...) ident)` which is identical to"] # [doc = " `split_borrow!(target, ident = &(...) ident)`. This shadows the existing partial reference with"] # [doc = " a new binding."] # [macro_export] macro_rules ! split_borrow { ($ target : ident = & ($ ($ parts : tt) *) $ from : ident) => { split_borrow ! ($ target , $ from = & ($ ($ parts) *) $ from) } ; ($ target : ident , $ rest : ident = & ($ ($ parts : tt) *) $ from : expr) => { # [allow (unused_variables , unused_mut)] let (mut $ target , mut $ rest) = ($ from) . split_borrow ::< partial ! (_ , $ ($ parts) *) , _ > () ; } ; }
};
}
