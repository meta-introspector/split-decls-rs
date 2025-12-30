// Generated macro for impl_506 (impl)
macro_rules! Depcrate_util_searchimpl_506 {
() => {
// Module: crate::util::search
// Provides: {"impl_506"}
// Dependencies: {}
impl Anchored { # [doc = " Returns true if and only if this anchor mode corresponds to an anchored"] # [doc = " search."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use aho_corasick::Anchored;"] # [doc = ""] # [doc = " assert!(!Anchored::No.is_anchored());"] # [doc = " assert!(Anchored::Yes.is_anchored());"] # [doc = " ```"] # [inline] pub fn is_anchored (& self) -> bool { matches ! (* self , Anchored :: Yes) } }
};
}
