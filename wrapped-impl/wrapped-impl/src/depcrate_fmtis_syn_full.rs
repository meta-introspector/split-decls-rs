// Generated macro for is_syn_full (function)
macro_rules! Depcrate_fmtis_syn_full {
() => {
// Module: crate::fmt
// Provides: {"is_syn_full"}
// Dependencies: {}
fn is_syn_full () -> bool { let test = quote ! ({ trait Trait { } }) ; match syn :: parse2 (test) { Ok (Expr :: Verbatim (_)) | Err (_) => false , Ok (Expr :: Block (_)) => true , Ok (_) => unreachable ! () , } }
};
}
