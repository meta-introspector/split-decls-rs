// Generated macro for impl_102 (impl)
macro_rules! Depcrate_typesimpl_102 {
() => {
// Module: crate::types
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > RFC822Name < 'a > { pub fn new (value : & 'a str) -> Option < Self > { let (local_part , domain) = value . split_once ('@') ? ; let local_part = IA5String :: new (local_part) ? ; for component in local_part . as_str () . split ('.') { if component . is_empty () || ! component . chars () . all (| c | c . is_ascii_alphanumeric () || ATEXT_CHARS . contains (c)) { return None ; } } Some (Self { mailbox : local_part , domain : DNSName :: new (domain) ? , }) } }
};
}
