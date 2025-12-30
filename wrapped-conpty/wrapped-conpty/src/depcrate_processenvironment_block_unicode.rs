// Generated macro for environment_block_unicode (function)
macro_rules! Depcrate_processenvironment_block_unicode {
() => {
// Module: crate::process
// Provides: {"environment_block_unicode"}
// Dependencies: {}
fn environment_block_unicode < 'a > (env : impl IntoIterator < Item = (& 'a OsStr , & 'a OsStr) > ,) -> Vec < u16 > { let mut b = Vec :: new () ; for (key , value) in env { b . extend (key . encode_wide ()) ; b . extend ("=" . encode_utf16 ()) ; b . extend (value . encode_wide ()) ; b . push (0) ; } if b . is_empty () { return vec ! [0 , 0] ; } b . push (0) ; b }
};
}
