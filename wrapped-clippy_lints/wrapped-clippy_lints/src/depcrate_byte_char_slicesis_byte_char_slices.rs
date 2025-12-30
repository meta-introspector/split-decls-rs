// Generated macro for is_byte_char_slices (function)
macro_rules! Depcrate_byte_char_slicesis_byte_char_slices {
() => {
// Module: crate::byte_char_slices
// Provides: {"is_byte_char_slices"}
// Dependencies: {}
fn is_byte_char_slices (expr : & Expr) -> Option < String > { if let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Not , expr) = & expr . kind { match & expr . kind { ExprKind :: Array (members) => { if members . is_empty () { return None ; } members . iter () . map (| member | match & member . kind { ExprKind :: Lit (Lit { kind : LitKind :: Byte , symbol , .. }) => Some (symbol . as_str ()) , _ => None , }) . map (| maybe_quote | match maybe_quote { Some ("\"") => Some ("\\\"") , Some ("\\'") => Some ("'") , other => other , }) . collect :: < Option < String > > () } , _ => None , } } else { None } }
};
}
