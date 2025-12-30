// Generated macro for impl_301 (impl)
macro_rules! Depcrate_rcdomimpl_301 {
() => {
// Module: crate::rcdom
// Provides: {"impl_301"}
// Dependencies: {}
impl Serializable for Handle { fn serialize < 'wr , Wr : Write > (& self , serializer : & mut Serializer < 'wr , Wr > , traversal_scope : TraversalScope) -> io :: Result < () > { let node = self . borrow () ; match (traversal_scope , & node . node) { (_ , & Element (ref name , _ , ref attrs)) => { if traversal_scope == IncludeNode { try ! (serializer . start_elem (name . clone () , attrs . iter () . map (| at | (& at . name , & at . value [..])))) ; } for handle in node . children . iter () { try ! (handle . clone () . serialize (serializer , IncludeNode)) ; } if traversal_scope == IncludeNode { try ! (serializer . end_elem (name . clone ())) ; } Ok (()) } (ChildrenOnly , & Document) => { for handle in node . children . iter () { try ! (handle . clone () . serialize (serializer , IncludeNode)) ; } Ok (()) } (ChildrenOnly , _) => Ok (()) , (IncludeNode , & Doctype (ref name , _ , _)) => serializer . write_doctype (& name) , (IncludeNode , & Text (ref text)) => serializer . write_text (& text) , (IncludeNode , & Comment (ref text)) => serializer . write_comment (& text) , (IncludeNode , & Document) => panic ! ("Can't serialize Document node itself") , } } }
};
}
