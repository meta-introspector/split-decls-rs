// Generated macro for FromVariantOptions (struct)
macro_rules! Depcrate_options_from_variantFromVariantOptions {
() => {
// Module: crate::options::from_variant
// Provides: {"FromVariantOptions"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct FromVariantOptions { pub base : OuterFrom , # [doc = " The field on the deriving struct into which the discriminant expression"] # [doc = " should be placed by the derived `FromVariant` impl."] pub discriminant : Option < Ident > , pub fields : Option < Ident > , pub supports : Option < DataShape > , }
};
}
