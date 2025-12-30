// Generated macro for componentwise_constructors_impl (macro)
macro_rules! Depcrate_base_constructioncomponentwise_constructors_impl {
() => {
// Module: crate::base::construction
// Provides: {"componentwise_constructors_impl"}
// Dependencies: {}
macro_rules ! componentwise_constructors_impl (($ ($ R : expr , $ C : expr , [$ ($ ($ args : ident) ,*) ;*] $ (;) *) *) => { $ (impl < T > Matrix < T , Const <$ R >, Const <$ C >, ArrayStorage < T , $ R , $ C >> { # [doc = " Initializes this matrix from its components."] # [inline] # [allow (clippy :: too_many_arguments)] pub const fn new ($ ($ ($ args : T) ,*) ,*) -> Self { unsafe { Self :: from_data_statically_unchecked (ArrayStorage (transpose_array ! [$ ($ ($ args) ,* ;) *])) } } }) * }) ;
};
}
