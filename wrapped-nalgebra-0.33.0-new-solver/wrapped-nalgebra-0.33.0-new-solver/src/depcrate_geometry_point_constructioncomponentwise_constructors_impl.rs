// Generated macro for componentwise_constructors_impl (macro)
macro_rules! Depcrate_geometry_point_constructioncomponentwise_constructors_impl {
() => {
// Module: crate::geometry::point_construction
// Provides: {"componentwise_constructors_impl"}
// Dependencies: {}
macro_rules ! componentwise_constructors_impl (($ ($ doc : expr ; $ Point : ident , $ Vector : ident , $ ($ args : ident :$ irow : expr) ,*) ;* $ (;) *) => { $ (impl < T : Scalar > $ Point < T > { # [doc = "Initializes this point from its components."] # [doc = "# Example\n```"] # [doc = $ doc] # [doc = "```"] # [inline] pub const fn new ($ ($ args : T) ,*) -> Self { Point { coords : $ Vector :: new ($ ($ args) ,*) } } }) * }) ;
};
}
