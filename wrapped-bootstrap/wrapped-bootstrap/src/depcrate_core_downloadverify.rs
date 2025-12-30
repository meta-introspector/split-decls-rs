// Generated macro for verify (function)
macro_rules! Depcrate_core_downloadverify {
() => {
// Module: crate::core::download
// Provides: {"verify"}
// Dependencies: {}
pub (crate) fn verify (exec_ctx : & ExecutionContext , path : & Path , expected : & str) -> bool { use sha2 :: Digest ; exec_ctx . verbose (| | { println ! ("verifying {}" , path . display ()) ; }) ; if exec_ctx . dry_run () { return false ; } let mut hasher = sha2 :: Sha256 :: new () ; let file = t ! (File :: open (path)) ; let mut reader = BufReader :: new (file) ; loop { let buffer = t ! (reader . fill_buf ()) ; let l = buffer . len () ; if l == 0 { break ; } hasher . update (buffer) ; reader . consume (l) ; } let checksum = hex_encode (hasher . finalize () . as_slice ()) ; let verified = checksum == expected ; if ! verified { println ! ("invalid checksum: \n\
            found:    {checksum}\n\
            expected: {expected}" ,) ; } verified }
};
}
