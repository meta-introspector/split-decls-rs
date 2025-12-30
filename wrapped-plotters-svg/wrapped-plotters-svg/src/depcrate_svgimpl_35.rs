// Generated macro for impl_35 (impl)
macro_rules! Depcrate_svgimpl_35 {
() => {
// Module: crate::svg
// Provides: {"impl_35"}
// Dependencies: {}
# [doc = " Used for opening a tag and then optionally writing some attributes. The expected workflow is"] # [doc = " to call [open_tag](AttrWriter::open_tag), then zero or more times calling"] # [doc = " [write_key](AttrWriter::write_key) followed optionally by `[write_value](AttrWriter::write_value)`"] # [doc = " and finally calling one of [close](AttrWriter::close) (to close a self-closing tag)"] # [doc = " or [finish_without_closing](AttrWriter::finish_without_closing) (to schedule writing"] impl < 'a > AttrWriter < 'a , Init > { fn open_tag (buf : & 'a mut String , tag : SVGTag , tag_stack : & 'a mut Vec < SVGTag >) -> Self { buf . push ('<') ; buf . push_str (tag . to_tag_name ()) ; AttrWriter { buf , tag , tag_stack , state : Default :: default () , } } fn write_key < 's > (& 's mut self , key : & str) -> AttrWriter < 's , Value > { self . buf . push (' ') ; self . buf . push_str (key) ; AttrWriter { buf : self . buf , tag : self . tag . clone () , tag_stack : self . tag_stack , state : Default :: default () , } } fn close (self) { self . buf . push_str ("/>\n") ; } fn finish_without_closing (self) { self . tag_stack . push (self . tag) ; self . buf . push_str (">\n") ; } }
};
}
