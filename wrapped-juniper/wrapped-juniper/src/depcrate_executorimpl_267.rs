// Generated macro for impl_267 (impl)
macro_rules! Depcrate_executorimpl_267 {
() => {
// Module: crate::executor
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'a , S1 , S2 , T , C > IntoResolvable < 'a , S2 , T , C > for FieldResult < (& 'a T :: Context , T) , S1 > where S1 : ScalarValue , S2 : ScalarValue , T : GraphQLValue < S2 > , { type Type = T ; fn into_resolvable (self , _ : & 'a C) -> FieldResult < Option < (& 'a T :: Context , T) > , S2 > { self . map (Some) . map_err (FieldError :: map_scalar_value) } }
};
}
