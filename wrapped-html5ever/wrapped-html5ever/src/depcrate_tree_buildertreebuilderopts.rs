// Generated macro for TreeBuilderOpts (struct)
macro_rules! Depcrate_tree_builderTreeBuilderOpts {
() => {
// Module: crate::tree_builder
// Provides: {"TreeBuilderOpts"}
// Dependencies: {}
# [doc = " Tree builder options, with an impl for Default."] # [derive (Copy , Clone)] pub struct TreeBuilderOpts { # [doc = " Report all parse errors described in the spec, at some"] # [doc = " performance penalty?  Default: false"] pub exact_errors : bool , # [doc = " Is scripting enabled?"] pub scripting_enabled : bool , # [doc = " Is this an `iframe srcdoc` document?"] pub iframe_srcdoc : bool , # [doc = " Should we drop the DOCTYPE (if any) from the tree?"] pub drop_doctype : bool , # [doc = " Obsolete, ignored."] pub ignore_missing_rules : bool , # [doc = " Initial TreeBuilder quirks mode. Default: NoQuirks"] pub quirks_mode : QuirksMode , }
};
}
