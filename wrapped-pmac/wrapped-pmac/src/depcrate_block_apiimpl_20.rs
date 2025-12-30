// Generated macro for impl_20 (impl)
macro_rules! Depcrate_block_apiimpl_20 {
() => {
// Module: crate::block_api
// Provides: {"impl_20"}
// Dependencies: {}
impl < C : PmacCipher , const LC_SIZE : usize > InnerInit for PmacCore < C , LC_SIZE > { # [inline] fn inner_init (cipher : C) -> Self { let mut l = Default :: default () ; cipher . encrypt_block (& mut l) ; let l_inv = C :: inv_dbl (l . clone ()) ; let l_cache = [() ; LC_SIZE] . map (| _ | { let next_l = C :: dbl (l . clone ()) ; core :: mem :: replace (& mut l , next_l) }) ; let state = PmacState { l_cache , l_inv , tag : Default :: default () , offset : Default :: default () , counter : 1 , } ; Self { cipher , state } } }
};
}
