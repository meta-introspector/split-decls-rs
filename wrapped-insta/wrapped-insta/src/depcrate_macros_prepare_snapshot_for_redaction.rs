// Generated macro for _prepare_snapshot_for_redaction (macro)
macro_rules! Depcrate_macros_prepare_snapshot_for_redaction {
() => {
// Module: crate::macros
// Provides: {"_prepare_snapshot_for_redaction"}
// Dependencies: {}
# [cfg (not (feature = "redactions"))] # [doc (hidden)] # [macro_export] macro_rules ! _prepare_snapshot_for_redaction { ($ value : expr , { $ ($ k : expr => $ v : expr) ,* } , $ format : ident) => { compile_error ! ("insta was compiled without redactions support. Enable the `redactions` feature.") } ; }
};
}
