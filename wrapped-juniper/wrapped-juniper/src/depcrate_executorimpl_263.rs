// Generated macro for impl_263 (impl)
macro_rules! Depcrate_executorimpl_263 {
() => {
// Module: crate::executor
// Provides: {"impl_263"}
// Dependencies: {}
impl < 'a , S , T , C > IntoResolvable < 'a , S , T , C > for T where T : GraphQLValue < S > , S : ScalarValue , T :: Context : FromContext < C > , { type Type = T ; fn into_resolvable (self , ctx : & 'a C) -> FieldResult < Option < (& 'a T :: Context , T) > , S > { Ok (Some ((FromContext :: from (ctx) , self))) } }
};
}
