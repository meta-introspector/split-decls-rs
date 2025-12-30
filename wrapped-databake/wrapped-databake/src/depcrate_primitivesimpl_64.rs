// Generated macro for impl_64 (impl)
macro_rules! Depcrate_primitivesimpl_64 {
() => {
// Module: crate::primitives
// Provides: {"impl_64"}
// Dependencies: {}
impl < T , const N : usize > Bake for [T ; N] where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { if core :: mem :: size_of :: < T > () == core :: mem :: size_of :: < u8 > () && core :: any :: type_name :: < T > () == core :: any :: type_name :: < u8 > () { let byte_string = proc_macro2 :: Literal :: byte_string (unsafe { core :: slice :: from_raw_parts (self . as_ptr () as * const u8 , N) }) ; return quote ! (*# byte_string) ; } let data = self . iter () . map (| d | d . bake (ctx)) ; quote ! { [# (# data) ,*] } } }
};
}
