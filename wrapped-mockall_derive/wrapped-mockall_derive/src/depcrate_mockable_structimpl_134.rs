// Generated macro for impl_134 (impl)
macro_rules! Depcrate_mockable_structimpl_134 {
() => {
// Module: crate::mockable_struct
// Provides: {"impl_134"}
// Dependencies: {}
impl Parse for TraitItemVFn { fn parse (input : ParseStream) -> syn :: parse :: Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; let vis : syn :: Visibility = input . parse () ? ; let mut tif : TraitItemFn = input . parse () ? ; tif . attrs = attrs ; Ok (Self { vis , tif }) } }
};
}
