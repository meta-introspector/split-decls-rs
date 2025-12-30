// Generated macro for _memoffset__field_check_union (macro)
macro_rules! Depcrate_raw_field_memoffset__field_check_union {
() => {
// Module: crate::raw_field
// Provides: {"_memoffset__field_check_union"}
// Dependencies: {}
# [cfg (not (allow_clippy))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__field_check_union { ($ type : path , $ field : tt) => { # [allow (unused_unsafe)] unsafe { let $ type { $ field : _ } ; } } ; }
};
}
