// Generated macro for impl_344 (impl)
macro_rules! Depcrate_ecimpl_344 {
() => {
// Module: crate::ec
// Provides: {"impl_344"}
// Dependencies: {}
impl EcGroup { # [doc = " Returns the group of a standard named curve."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " use openssl::nid::Nid;"] # [doc = " use openssl::ec::{EcGroup, EcKey};"] # [doc = ""] # [doc = " let nid = Nid::X9_62_PRIME256V1; // NIST P-256 curve"] # [doc = " let group = EcGroup::from_curve_name(nid)?;"] # [doc = " let key = EcKey::generate(&group)?;"] # [doc = " # Ok(()) }"] # [doc = " ```"] # [corresponds (EC_GROUP_new_by_curve_name)] pub fn from_curve_name (nid : Nid) -> Result < EcGroup , ErrorStack > { unsafe { init () ; cvt_p (ffi :: EC_GROUP_new_by_curve_name (nid . as_raw ())) . map (EcGroup) } } # [doc = " Returns the group for given parameters"] # [corresponds (EC_GROUP_new_curve_GFp)] pub fn from_components (p : BigNum , a : BigNum , b : BigNum , ctx : & mut BigNumContextRef ,) -> Result < EcGroup , ErrorStack > { unsafe { cvt_p (ffi :: EC_GROUP_new_curve_GFp (p . as_ptr () , a . as_ptr () , b . as_ptr () , ctx . as_ptr () ,)) . map (EcGroup) } } }
};
}
