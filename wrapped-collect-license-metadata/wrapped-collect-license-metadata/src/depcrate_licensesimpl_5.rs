// Generated macro for impl_5 (impl)
macro_rules! Depcrate_licensesimpl_5 {
() => {
// Module: crate::licenses
// Provides: {"impl_5"}
// Dependencies: {}
impl LicensesInterner { pub (crate) fn new () -> Self { LicensesInterner { by_id : Vec :: new () , by_struct : HashMap :: new () } } pub (crate) fn intern (& mut self , mut license : License) -> LicenseId { license . simplify () ; if let Some (id) = self . by_struct . get (& license) { LicenseId (* id) } else { let id = self . by_id . len () ; self . by_id . push (license . clone ()) ; self . by_struct . insert (license , id) ; LicenseId (id) } } pub (crate) fn resolve (& self , id : LicenseId) -> & License { & self . by_id [id . 0] } }
};
}
