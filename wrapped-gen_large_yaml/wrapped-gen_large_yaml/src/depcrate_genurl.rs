// Generated macro for url (function)
macro_rules! Depcrate_genurl {
() => {
// Module: crate::gen
// Provides: {"url"}
// Dependencies: {}
# [doc = " Generate a random URL."] pub fn url (rng : & mut SmallRng , scheme : & str , n_paths_lo : usize , n_paths_hi : usize , path_len_lo : usize , path_len_hi : usize , extension : Option < & str > ,) -> String { let mut string = format ! ("{scheme}://example.com") ; for _ in 0 .. rng . gen_range (n_paths_lo .. n_paths_hi) { string . push ('/') ; string . push_str (& alnum_string (rng , path_len_lo , path_len_hi)) ; } if let Some (extension) = extension { string . push ('.') ; string . push_str (extension) ; } string }
};
}
