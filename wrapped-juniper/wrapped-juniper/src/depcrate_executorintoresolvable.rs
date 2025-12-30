// Generated macro for IntoResolvable (trait)
macro_rules! Depcrate_executorIntoResolvable {
() => {
// Module: crate::executor
// Provides: {"IntoResolvable"}
// Dependencies: {}
# [doc (hidden)] pub trait IntoResolvable < 'a , S , T , C > where T : GraphQLValue < S > , S : ScalarValue , { type Type ; # [doc (hidden)] fn into_resolvable (self , ctx : & 'a C) -> FieldResult < Option < (& 'a T :: Context , T) > , S > ; }
};
}
