// Generated macro for MD_LINK_INLINE (static)
macro_rules! Depcrate_std_linksMD_LINK_INLINE {
() => {
// Module: crate::std_links
// Provides: {"MD_LINK_INLINE"}
// Dependencies: {}
# [doc = " Regex for a markdown inline link, like `[foo](bar)`."] static MD_LINK_INLINE : Lazy < Regex > = Lazy :: new (| | Regex :: new (r"(?s)(\[.+\])(\(.+\))") . unwrap ()) ;
};
}
