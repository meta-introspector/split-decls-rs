// Generated macro for HtmlScanGuard (struct)
macro_rules! Depcrate_parseHtmlScanGuard {
() => {
// Module: crate::parse
// Provides: {"HtmlScanGuard"}
// Dependencies: {}
# [doc = " A struct containing information on the reachability of certain inline HTML"] # [doc = " elements. In particular, for cdata elements (`<![CDATA[`), processing"] # [doc = " elements (`<?`) and declarations (`<!DECLARATION`). The respectives usizes"] # [doc = " represent the indices before which a scan will always fail and can hence"] # [doc = " be skipped."] # [derive (Clone , Default)] pub (crate) struct HtmlScanGuard { pub cdata : usize , pub processing : usize , pub declaration : usize , pub comment : usize , }
};
}
