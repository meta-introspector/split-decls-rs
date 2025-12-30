// Generated macro for impl_265 (impl)
macro_rules! Depcrate_executorimpl_265 {
() => {
// Module: crate::executor
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'a , S , T , C > IntoResolvable < 'a , S , T , C > for (& 'a T :: Context , T) where S : ScalarValue , T : GraphQLValue < S > , { type Type = T ; fn into_resolvable (self , _ : & 'a C) -> FieldResult < Option < (& 'a T :: Context , T) > , S > { Ok (Some (self)) } }
};
}
