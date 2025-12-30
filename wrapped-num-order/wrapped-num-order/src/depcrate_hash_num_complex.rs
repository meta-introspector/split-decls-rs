// Generated macro for _num_complex (module)
macro_rules! Depcrate_hash_num_complex {
() => {
// Module: crate::hash
// Provides: {"_num_complex"}
// Dependencies: {}
# [cfg (feature = "num-complex")] mod _num_complex { use super :: * ; use num_complex :: Complex ; macro_rules ! impl_complex_hash_for_float { ($ ($ float : ty) *) => ($ (impl NumHash for Complex <$ float > { fn num_hash < H : Hasher > (& self , state : & mut H) { let a = self . re . fhash () ; let b = self . im . fhash () ; let bterm = if b >= 0 { let pb = MInt :: new (b as u128 , & M127U) * PROOT ; - ((pb * pb) . residue () as i128) } else { let pb = MInt :: new ((- b) as u128 , & M127U) * PROOT ; (pb * pb) . residue () as i128 } ; (a + bterm) . num_hash (state) } }) *) ; } impl_complex_hash_for_float ! (f32 f64) ; }
};
}
