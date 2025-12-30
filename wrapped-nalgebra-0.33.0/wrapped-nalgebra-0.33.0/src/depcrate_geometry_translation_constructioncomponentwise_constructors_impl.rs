// Generated macro for componentwise_constructors_impl (macro)
macro_rules! Depcrate_geometry_translation_constructioncomponentwise_constructors_impl {
() => {
// Module: crate::geometry::translation_construction
// Provides: {"componentwise_constructors_impl"}
// Dependencies: {}
macro_rules ! componentwise_constructors_impl (($ ($ doc : expr ; $ D : expr , $ ($ args : ident :$ irow : expr) ,*) ;* $ (;) *) => { $ (impl < T > Translation < T , $ D > { # [doc = "Initializes this translation from its components."] # [doc = "# Example\n```"] # [doc = $ doc] # [doc = "```"] # [inline] pub const fn new ($ ($ args : T) ,*) -> Self { Self { vector : SVector ::< T , $ D >:: new ($ ($ args) ,*) } } }) * }) ;
};
}
