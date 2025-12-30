// Generated macro for impl_59 (impl)
macro_rules! Depcrate_enumeratedimpl_59 {
() => {
// Module: crate::enumerated
// Provides: {"impl_59"}
// Dependencies: {}
impl EnumeratedVariant { # [doc = " Create a new [`ChoiceVariant`] from the input [`Variant`]."] fn new (input : & Variant) -> syn :: Result < Self > { for attr in & input . attrs { if attr . path () . is_ident (ATTR_NAME) { abort ! (attr , "`asn1` attribute is not allowed on fields of `Enumerated` types") ; } } match & input . discriminant { Some ((_ , Expr :: Lit (ExprLit { lit : Lit :: Int (discriminant) , .. }) ,)) => Ok (Self { ident : input . ident . clone () , discriminant : discriminant . clone () , }) , Some ((_ , other)) => abort ! (other , "invalid discriminant for `Enumerated`") , None => abort ! (input , "`Enumerated` variant has no discriminant") , } } # [doc = " Write the body for the derived [`TryFrom`] impl."] pub fn to_try_from_tokens (& self) -> TokenStream { let ident = & self . ident ; let discriminant = & self . discriminant ; quote ! { # discriminant => Ok (Self ::# ident) , } } }
};
}
