// Generated macro for deserialize_variant (macro)
macro_rules! Depcrate_serde_helpersdeserialize_variant {
() => {
// Module: crate::serde_helpers
// Provides: {"deserialize_variant"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! deserialize_variant { ($ de : expr , $ enum : tt , $ variant : ident { $ ($ (# [$ meta : meta]) * $ field : ident : $ typ : ty) ,* $ (,) ? }) => ({ let var = { # [derive (serde :: Deserialize)] struct $ variant { $ ($ (# [$ meta]) * $ field : $ typ ,) * } <$ variant >:: deserialize ($ de) ? } ; use $ enum :: *; $ variant { $ ($ field : var .$ field ,) * } }) ; ($ de : expr , $ enum : tt , $ variant : ident ($ typ : ty)) => ({ let var = <$ typ >:: deserialize ($ de) ?; <$ enum > :: $ variant (var) }) ; ($ de : expr , $ enum : tt , $ variant : ident) => ({ serde :: de :: IgnoredAny :: deserialize ($ de) ?; <$ enum > :: $ variant }) ; }
};
}
