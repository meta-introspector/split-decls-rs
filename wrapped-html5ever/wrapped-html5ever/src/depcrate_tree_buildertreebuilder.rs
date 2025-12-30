// Generated macro for TreeBuilder (struct)
macro_rules! Depcrate_tree_builderTreeBuilder {
() => {
// Module: crate::tree_builder
// Provides: {"TreeBuilder"}
// Dependencies: {}
# [doc = " The HTML tree builder."] pub struct TreeBuilder < Handle , Sink > { # [doc = " Options controlling the behavior of the tree builder."] opts : TreeBuilderOpts , # [doc = " Consumer of tree modifications."] sink : Sink , # [doc = " Insertion mode."] mode : InsertionMode , # [doc = " Original insertion mode, used by Text and InTableText modes."] orig_mode : Option < InsertionMode > , # [doc = " Stack of template insertion modes."] template_modes : Vec < InsertionMode > , # [doc = " Pending table character tokens."] pending_table_text : Vec < (SplitStatus , StrTendril) > , # [doc = " Quirks mode as set by the parser."] # [doc = " FIXME: can scripts etc. change this?"] quirks_mode : QuirksMode , # [doc = " The document node, which is created by the sink."] doc_handle : Handle , # [doc = " Stack of open elements, most recently added at end."] open_elems : Vec < Handle > , # [doc = " List of active formatting elements."] active_formatting : Vec < FormatEntry < Handle > > , # [doc = " Head element pointer."] head_elem : Option < Handle > , # [doc = " Form element pointer."] form_elem : Option < Handle > , # [doc = " Next state change for the tokenizer, if any."] next_tokenizer_state : Option < tokenizer :: states :: State > , # [doc = " Frameset-ok flag."] frameset_ok : bool , # [doc = " Ignore a following U+000A LINE FEED?"] ignore_lf : bool , # [doc = " Is foster parenting enabled?"] foster_parenting : bool , # [doc = " The context element for the fragment parsing algorithm."] context_elem : Option < Handle > , }
};
}
