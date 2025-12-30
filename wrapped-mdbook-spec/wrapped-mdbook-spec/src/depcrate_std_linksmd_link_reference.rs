// Generated macro for MD_LINK_REFERENCE (static)
macro_rules! Depcrate_std_linksMD_LINK_REFERENCE {
() => {
// Module: crate::std_links
// Provides: {"MD_LINK_REFERENCE"}
// Dependencies: {}
# [doc = " Regex for a markdown reference link, like `[foo][bar]`."] static MD_LINK_REFERENCE : Lazy < Regex > = Lazy :: new (| | Regex :: new (r"(?s)(\[.+\])(\[.*\])") . unwrap ()) ;
};
}
