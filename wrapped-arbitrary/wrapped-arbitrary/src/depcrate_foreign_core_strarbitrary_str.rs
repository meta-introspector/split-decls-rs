// Generated macro for arbitrary_str (function)
macro_rules! Depcrate_foreign_core_strarbitrary_str {
() => {
// Module: crate::foreign::core::str
// Provides: {"arbitrary_str"}
// Dependencies: {}
fn arbitrary_str < 'a > (u : & mut Unstructured < 'a > , size : usize) -> Result < & 'a str > { match str :: from_utf8 (u . peek_bytes (size) . unwrap ()) { Ok (s) => { u . bytes (size) . unwrap () ; Ok (s) } Err (e) => { let i = e . valid_up_to () ; let valid = u . bytes (i) . unwrap () ; let s = unsafe { debug_assert ! (str :: from_utf8 (valid) . is_ok ()) ; str :: from_utf8_unchecked (valid) } ; Ok (s) } } }
};
}
