// Generated macro for default_struct_substructure (function)
macro_rules! Depcrate_deriving_defaultdefault_struct_substructure {
() => {
// Module: crate::deriving::default
// Provides: {"default_struct_substructure"}
// Dependencies: {}
fn default_struct_substructure (cx : & ExtCtxt < '_ > , trait_span : Span , substr : & Substructure < '_ > , summary : & StaticFields ,) -> BlockOrExpr { let expr = match summary { Unnamed (_ , IsTuple :: No) => cx . expr_ident (trait_span , substr . type_ident) , Unnamed (fields , IsTuple :: Yes) => { let exprs = fields . iter () . map (| sp | default_call (cx , * sp)) . collect () ; cx . expr_call_ident (trait_span , substr . type_ident , exprs) } Named (fields) => { let default_fields = fields . iter () . map (| (ident , span , default_val) | { let value = match default_val { None => default_call (cx , * span) , Some (val) => { cx . expr (val . value . span , ast :: ExprKind :: ConstBlock (val . clone ())) } } ; cx . field_imm (* span , * ident , value) }) . collect () ; cx . expr_struct_ident (trait_span , substr . type_ident , default_fields) } } ; BlockOrExpr :: new_expr (expr) }
};
}
