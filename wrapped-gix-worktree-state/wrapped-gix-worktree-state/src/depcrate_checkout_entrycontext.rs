// Generated macro for Context (struct)
macro_rules! Depcrate_checkout_entryContext {
() => {
// Module: crate::checkout::entry
// Provides: {"Context"}
// Dependencies: {}
pub struct Context < 'a , Find > { pub objects : & 'a mut Find , pub path_cache : & 'a mut Stack , pub filters : & 'a mut gix_filter :: Pipeline , pub buf : & 'a mut Vec < u8 > , }
};
}
