// Generated macro for is_doc_attr (function)
macro_rules! Depcrate_missing_docis_doc_attr {
() => {
// Module: crate::missing_doc
// Provides: {"is_doc_attr"}
// Dependencies: {}
fn is_doc_attr (attr : & Attribute) -> bool { match attr { Attribute :: Parsed (AttributeKind :: DocComment { .. }) => true , Attribute :: Unparsed (attr) if let [ident] = & * attr . path . segments && ident . name == sym :: doc => { matches ! (attr . args , AttrArgs :: Eq { .. }) } , _ => false , } }
};
}
