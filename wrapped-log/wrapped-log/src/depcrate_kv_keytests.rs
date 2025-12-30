// Generated macro for tests (module)
macro_rules! Depcrate_kv_keytests {
() => {
// Module: crate::kv::key
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn key_from_string () { assert_eq ! ("a key" , Key :: from_str ("a key") . as_str ()) ; } # [test] fn key_to_borrowed () { assert_eq ! ("a key" , Key :: from_str ("a key") . to_borrowed_str () . unwrap ()) ; } }
};
}
