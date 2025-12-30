// Generated macro for impl_657 (impl)
macro_rules! Depcrate_options_shapeimpl_657 {
() => {
// Module: crate::options::shape
// Provides: {"impl_657"}
// Dependencies: {}
impl ToTokens for DataShape { fn to_tokens (& self , tokens : & mut TokenStream) { let Self { any , named , tuple , unit , newtype , .. } = * self ; let shape_path : syn :: Path = parse_quote ! (:: darling :: util :: Shape) ; let mut shapes = vec ! [] ; if any || named { shapes . push (quote ! (# shape_path :: Named)) ; } if any || tuple { shapes . push (quote ! (# shape_path :: Tuple)) ; } if any || newtype { shapes . push (quote ! (# shape_path :: Newtype)) ; } if any || unit { shapes . push (quote ! (# shape_path :: Unit)) ; } tokens . append_all (quote ! { :: darling :: util :: ShapeSet :: new (vec ! [# (# shapes) ,*]) }) ; } }
};
}
