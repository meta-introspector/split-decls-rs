// Generated macro for specialize_for_lengths (macro)
macro_rules! Depcrate_strspecialize_for_lengths {
() => {
// Module: crate::str
// Provides: {"specialize_for_lengths"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] macro_rules ! specialize_for_lengths { ($ separator : expr , $ target : expr , $ iter : expr ; $ ($ num : expr) ,*) => { { let mut target = $ target ; let iter = $ iter ; let sep_bytes = $ separator ; match $ separator . len () { $ ($ num => { for s in iter { copy_slice_and_advance ! (target , sep_bytes) ; let content_bytes = s . borrow () . as_ref () ; copy_slice_and_advance ! (target , content_bytes) ; } } ,) * _ => { for s in iter { copy_slice_and_advance ! (target , sep_bytes) ; let content_bytes = s . borrow () . as_ref () ; copy_slice_and_advance ! (target , content_bytes) ; } } } target } } }
};
}
