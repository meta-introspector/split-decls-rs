// Generated macro for impl_352 (impl)
macro_rules! Depcrate_ecimpl_352 {
() => {
// Module: crate::ec
// Provides: {"impl_352"}
// Dependencies: {}
impl < T > EcKeyRef < T > where T : HasParams , { # [doc = " Returns the key's group."] # [corresponds (EC_KEY_get0_group)] pub fn group (& self) -> & EcGroupRef { unsafe { let ptr = ffi :: EC_KEY_get0_group (self . as_ptr ()) ; EcGroupRef :: from_const_ptr (ptr) } } # [doc = " Checks the key for validity."] # [corresponds (EC_KEY_check_key)] pub fn check_key (& self) -> Result < () , ErrorStack > { unsafe { cvt (ffi :: EC_KEY_check_key (self . as_ptr ())) . map (| _ | ()) } } }
};
}
