// Generated macro for tests (module)
macro_rules! Depcrate_processtests {
() => {
// Module: crate::process
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn env_block_test () { let tests = [(vec ! [] , "\0\0") , (vec ! [(OsStr :: new ("asd") , OsStr :: new ("qwe"))] , "asd=qwe\0\0") , (vec ! [(OsStr :: new ("asd") , OsStr :: new ("qwe")) , (OsStr :: new ("zxc") , OsStr :: new ("123")) ,] , "asd=qwe\0zxc=123\0\0" ,) ,] ; for (m , expected) in tests { let env = environment_block_unicode (m) ; let expected = str_to_utf16 (expected) ; assert_eq ! (env , expected ,) ; } } fn str_to_utf16 (s : impl AsRef < str >) -> Vec < u16 > { s . as_ref () . encode_utf16 () . collect () } }
};
}
