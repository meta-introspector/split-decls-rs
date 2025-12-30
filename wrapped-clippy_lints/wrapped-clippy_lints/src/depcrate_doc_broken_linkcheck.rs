// Generated macro for check (function)
macro_rules! Depcrate_doc_broken_linkcheck {
() => {
// Module: crate::doc::broken_link
// Provides: {"check"}
// Dependencies: {}
# [doc = " Scan and report broken link on documents."] # [doc = " It ignores false positives detected by `pulldown_cmark`, and only"] # [doc = " warns users when the broken link is consider a URL."] pub fn check (cx : & LateContext < '_ > , bl : & PullDownBrokenLink < '_ > , doc : & str , fragments : & [DocFragment]) { warn_if_broken_link (cx , bl , doc , fragments) ; }
};
}
