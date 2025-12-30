// Generated macro for impl_38 (impl)
macro_rules! Depcrate_serializeimpl_38 {
() => {
// Module: crate::serialize
// Provides: {"impl_38"}
// Dependencies: {}
impl < Wr : Write > HtmlSerializer < Wr > { pub fn new (writer : Wr , opts : SerializeOpts) -> Self { let html_name = match opts . traversal_scope { TraversalScope :: IncludeNode | TraversalScope :: ChildrenOnly (None) => None , TraversalScope :: ChildrenOnly (Some (ref n)) => Some (tagname (n)) , } ; HtmlSerializer { writer , opts , stack : vec ! [ElemInfo { html_name , ignore_children : false , }] , } } fn parent (& mut self) -> & mut ElemInfo { if self . stack . is_empty () { if self . opts . create_missing_parent { warn ! ("ElemInfo stack empty, creating new parent") ; self . stack . push (Default :: default ()) ; } else { panic ! ("no parent ElemInfo") } } self . stack . last_mut () . unwrap () } fn write_escaped (& mut self , text : & str , attr_mode : bool) -> io :: Result < () > { for c in text . chars () { match c { '&' => self . writer . write_all (b"&amp;") , '\u{00A0}' => self . writer . write_all (b"&nbsp;") , '"' if attr_mode => self . writer . write_all (b"&quot;") , '<' if ! attr_mode => self . writer . write_all (b"&lt;") , '>' if ! attr_mode => self . writer . write_all (b"&gt;") , c => self . writer . write_fmt (format_args ! ("{c}")) , } ? ; } Ok (()) } }
};
}
