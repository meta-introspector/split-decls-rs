// Generated macro for hash_stuff (function)
macro_rules! Depcrate_spanhash_stuff {
() => {
// Module: crate::span
// Provides: {"hash_stuff"}
// Dependencies: {}
fn hash_stuff () -> impl Xof { let span = Span :: call_site () ; let mut hasher = Shake :: v256 () ; hasher . update (get_seed ()) ; hasher . update (& format ! ("{:?}" , span) . as_bytes ()) ; hasher }
};
}
