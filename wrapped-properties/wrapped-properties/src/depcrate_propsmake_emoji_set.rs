// Generated macro for make_emoji_set (macro)
macro_rules! Depcrate_propsmake_emoji_set {
() => {
// Module: crate::props
// Provides: {"make_emoji_set"}
// Dependencies: {}
macro_rules ! make_emoji_set { (ident : $ ident : ident ; data_marker : $ data_marker : ty ; singleton : $ singleton : ident ; $ (# [$ doc : meta]) +) => { $ (# [$ doc]) + # [derive (Debug)] # [non_exhaustive] pub struct $ ident ; impl crate :: private :: Sealed for $ ident { } impl EmojiSet for $ ident { type DataMarker = $ data_marker ; # [cfg (feature = "compiled_data")] const SINGLETON : &'static crate :: provider :: PropertyUnicodeSet <'static > = & crate :: provider :: Baked ::$ singleton ; } } }
};
}
