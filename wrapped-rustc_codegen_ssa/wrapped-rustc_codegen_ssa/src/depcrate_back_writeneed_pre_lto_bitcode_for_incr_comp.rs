// Generated macro for need_pre_lto_bitcode_for_incr_comp (function)
macro_rules! Depcrate_back_writeneed_pre_lto_bitcode_for_incr_comp {
() => {
// Module: crate::back::write
// Provides: {"need_pre_lto_bitcode_for_incr_comp"}
// Dependencies: {}
fn need_pre_lto_bitcode_for_incr_comp (sess : & Session) -> bool { if sess . opts . incremental . is_none () { return false ; } match sess . lto () { Lto :: No => false , Lto :: Fat | Lto :: Thin | Lto :: ThinLocal => true , } }
};
}
