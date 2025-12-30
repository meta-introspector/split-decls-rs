// Generated macro for impl_2064 (impl)
macro_rules! Depcrate_wtf8impl_2064 {
() => {
// Module: crate::wtf8
// Provides: {"impl_2064"}
// Dependencies: {}
# [cfg (not (test))] impl Wtf8 { # [rustc_allow_incoherent_impl] pub fn to_owned (& self) -> Wtf8Buf { to_owned (self) } # [rustc_allow_incoherent_impl] pub fn clone_into (& self , buf : & mut Wtf8Buf) { clone_into (self , buf) } # [rustc_allow_incoherent_impl] pub fn to_string_lossy (& self) -> Cow < '_ , str > { to_string_lossy (self) } # [rustc_allow_incoherent_impl] pub fn into_box (& self) -> Box < Wtf8 > { let boxed : Box < [u8] > = self . as_bytes () . into () ; unsafe { mem :: transmute (boxed) } } # [rustc_allow_incoherent_impl] pub fn empty_box () -> Box < Wtf8 > { let boxed : Box < [u8] > = Default :: default () ; unsafe { mem :: transmute (boxed) } } # [cfg (target_has_atomic = "ptr")] # [rustc_allow_incoherent_impl] pub fn into_arc (& self) -> Arc < Wtf8 > { let arc : Arc < [u8] > = Arc :: from (self . as_bytes ()) ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const Wtf8) } } # [rustc_allow_incoherent_impl] pub fn into_rc (& self) -> Rc < Wtf8 > { let rc : Rc < [u8] > = Rc :: from (self . as_bytes ()) ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const Wtf8) } } # [inline] # [rustc_allow_incoherent_impl] pub fn to_ascii_lowercase (& self) -> Wtf8Buf { Wtf8Buf { bytes : self . as_bytes () . to_ascii_lowercase () , is_known_utf8 : false } } # [inline] # [rustc_allow_incoherent_impl] pub fn to_ascii_uppercase (& self) -> Wtf8Buf { Wtf8Buf { bytes : self . as_bytes () . to_ascii_uppercase () , is_known_utf8 : false } } }
};
}
