// Generated macro for MD_LINK_SHORTCUT (static)
macro_rules! Depcrate_std_linksMD_LINK_SHORTCUT {
() => {
// Module: crate::std_links
// Provides: {"MD_LINK_SHORTCUT"}
// Dependencies: {}
# [doc = " Regex for a markdown shortcut link, like `[foo]`."] static MD_LINK_SHORTCUT : Lazy < Regex > = Lazy :: new (| | Regex :: new (r"(?s)(\[.+\])") . unwrap ()) ;
};
}
