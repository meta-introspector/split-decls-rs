// Generated macro for impl_41 (impl)
macro_rules! Depcrate_converterimpl_41 {
() => {
// Module: crate::converter
// Provides: {"impl_41"}
// Dependencies: {}
impl < B , T > Bake for IteratorAsRefSlice < B , T > where for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { let mut inner = TokenStream :: new () ; for e in self . 0 . into_iter () { let e = e . bake (ctx) ; inner . extend (quote ! { # e , }) ; } quote ! { & [# inner] } } }
};
}
