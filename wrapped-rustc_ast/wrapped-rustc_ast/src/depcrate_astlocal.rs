// Generated macro for Local (struct)
macro_rules! Depcrate_astLocal {
() => {
// Module: crate::ast
// Provides: {"Local"}
// Dependencies: {}
# [doc = " Local represents a `let` statement, e.g., `let <pat>:<ty> = <expr>;`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Local { pub id : NodeId , pub super_ : Option < Span > , pub pat : Box < Pat > , pub ty : Option < Box < Ty > > , pub kind : LocalKind , pub span : Span , pub colon_sp : Option < Span > , pub attrs : AttrVec , pub tokens : Option < LazyAttrTokenStream > , }
};
}
