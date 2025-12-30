// Generated macro for impl_268 (impl)
macro_rules! Depcrate_executorimpl_268 {
() => {
// Module: crate::executor
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'a , S1 , S2 , T , C > IntoResolvable < 'a , S2 , Option < T > , C > for FieldResult < Option < (& 'a T :: Context , T) > , S1 > where S1 : ScalarValue , S2 : ScalarValue , T : GraphQLValue < S2 > , { type Type = T ; fn into_resolvable (self , _ : & 'a C) -> FieldResult < Option < (& 'a T :: Context , Option < T >) > , S2 > { self . map (| o | o . map (| (ctx , v) | (ctx , Some (v)))) . map_err (FieldError :: map_scalar_value) } }
};
}
