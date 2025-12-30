// Generated macro for impl_64 (impl)
macro_rules! Depcrate_macros_reflectimpl_64 {
() => {
// Module: crate::macros::reflect
// Provides: {"impl_64"}
// Dependencies: {}
impl < S , T > BaseSubTypes < S > for (& T :: Context , T) where S : ScalarValue , T : BaseSubTypes < S > + GraphQLValue < S > , { const NAMES : Types = T :: NAMES ; }
};
}
