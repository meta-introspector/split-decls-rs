// Generated macro for impl_264 (impl)
macro_rules! Depcrate_executorimpl_264 {
() => {
// Module: crate::executor
// Provides: {"impl_264"}
// Dependencies: {}
impl < 'a , S , T , C , E : IntoFieldError < S > > IntoResolvable < 'a , S , T , C > for Result < T , E > where S : ScalarValue , T : GraphQLValue < S > , T :: Context : FromContext < C > , { type Type = T ; fn into_resolvable (self , ctx : & 'a C) -> FieldResult < Option < (& 'a T :: Context , T) > , S > { self . map (| v : T | Some ((< T :: Context as FromContext < C > > :: from (ctx) , v))) . map_err (IntoFieldError :: into_field_error) } }
};
}
