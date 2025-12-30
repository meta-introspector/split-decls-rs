// Generated macro for impl_266 (impl)
macro_rules! Depcrate_executorimpl_266 {
() => {
// Module: crate::executor
// Provides: {"impl_266"}
// Dependencies: {}
impl < 'a , S , T , C > IntoResolvable < 'a , S , Option < T > , C > for Option < (& 'a T :: Context , T) > where S : ScalarValue , T : GraphQLValue < S > , { type Type = T ; fn into_resolvable (self , _ : & 'a C) -> FieldResult < Option < (& 'a T :: Context , Option < T >) > , S > { Ok (self . map (| (ctx , v) | (ctx , Some (v)))) } }
};
}
