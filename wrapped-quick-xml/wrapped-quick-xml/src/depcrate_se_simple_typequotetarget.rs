// Generated macro for QuoteTarget (enum)
macro_rules! Depcrate_se_simple_typeQuoteTarget {
() => {
// Module: crate::se::simple_type
// Provides: {"QuoteTarget"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum QuoteTarget { # [doc = " Escape data for a text content. No additional escape symbols"] Text , # [doc = " Escape data for a double-quoted attribute. `\"` always escaped"] DoubleQAttr , # [doc = " Escape data for a single-quoted attribute. `'` always escaped"] SingleQAttr , # [doc = " Escape data for a CDATA content. No escaping for `&` and `>`, but split"] # [doc = " content on `]]>` and make several CDATA sections"] CData , }
};
}
