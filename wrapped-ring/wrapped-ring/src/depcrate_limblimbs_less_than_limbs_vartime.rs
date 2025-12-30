// Generated macro for limbs_less_than_limbs_vartime (function)
macro_rules! Depcrate_limblimbs_less_than_limbs_vartime {
() => {
// Module: crate::limb
// Provides: {"limbs_less_than_limbs_vartime"}
// Dependencies: {}
# [inline] pub fn limbs_less_than_limbs_vartime (a : & [Limb] , b : & [Limb]) -> Result < bool , LenMismatchError > { let r = limbs_less_than_limbs (a , b) ? ; Ok (r . leak ()) }
};
}
