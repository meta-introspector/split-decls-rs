// Generated macro for impl_50 (impl)
macro_rules! Depcrate_frontend_databakeimpl_50 {
() => {
// Module: crate::frontend::databake
// Provides: {"impl_50"}
// Dependencies: {}
impl < B > Bake for & Pattern < B > where B : PatternBackend , for < 'b > & 'b B :: Store : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("icu_pattern") ; let store = (& self . store) . bake (ctx) ; let b = if TypeId :: of :: < B > () == TypeId :: of :: < SinglePlaceholder > () { quote ! (icu_pattern :: SinglePlaceholder) } else if TypeId :: of :: < B > () == TypeId :: of :: < DoublePlaceholder > () { quote ! (icu_pattern :: DoublePlaceholder) } else { unreachable ! ("all impls of sealed trait PatternBackend should be covered") } ; quote ! { icu_pattern :: Pattern ::<# b >:: from_ref_store_unchecked (# store) } } }
};
}
