// Generated macro for generate (function)
macro_rules! Depcrate_randgenerate {
() => {
// Module: crate::rand
// Provides: {"generate"}
// Dependencies: {}
# [doc = " Generate the new random value using `rng`."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if unable to fill buffer."] # [inline] pub fn generate < T : RandomlyConstructable > (rng : & dyn SecureRandom ,) -> Result < Random < T > , Unspecified > { let mut r = T :: zero () ; rng . fill (r . as_mut_bytes ()) ? ; Ok (Random (r)) }
};
}
