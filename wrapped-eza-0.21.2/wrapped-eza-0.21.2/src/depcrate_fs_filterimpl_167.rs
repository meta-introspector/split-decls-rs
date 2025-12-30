// Generated macro for impl_167 (impl)
macro_rules! Depcrate_fs_filterimpl_167 {
() => {
// Module: crate::fs::filter
// Provides: {"impl_167"}
// Dependencies: {}
impl IgnorePatterns { # [doc = " Create a new list from the input glob strings, turning the inputs that"] # [doc = " are valid glob patterns into an `IgnorePatterns`. The inputs that"] # [doc = " don’t parse correctly are returned separately."] pub fn parse_from_iter < 'a , I : IntoIterator < Item = & 'a str > > (iter : I ,) -> (Self , Vec < glob :: PatternError >) { let iter = iter . into_iter () ; let mut patterns = match iter . size_hint () { (_ , Some (count)) => Vec :: with_capacity (count) , _ => Vec :: new () , } ; let mut errors = Vec :: new () ; for input in iter { match glob :: Pattern :: new (input) { Ok (pat) => patterns . push (pat) , Err (e) => errors . push (e) , } } (Self { patterns } , errors) } # [doc = " Create a new empty set of patterns that matches nothing."] pub fn empty () -> Self { Self { patterns : Vec :: new () , } } # [doc = " Test whether the given file should be hidden from the results."] fn is_ignored (& self , file : & str) -> bool { self . patterns . iter () . any (| p | p . matches (file)) } }
};
}
