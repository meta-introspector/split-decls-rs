// Generated macro for _memoffset__let_base_ptr (macro)
macro_rules! Depcrate_offset_of_memoffset__let_base_ptr {
() => {
// Module: crate::offset_of
// Provides: {"_memoffset__let_base_ptr"}
// Dependencies: {}
# [cfg (not (maybe_uninit))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__let_base_ptr { ($ name : ident , $ type : ty) => { let $ name = $ crate :: __priv :: mem :: align_of ::<$ type > () as * const $ type ; } ; }
};
}
