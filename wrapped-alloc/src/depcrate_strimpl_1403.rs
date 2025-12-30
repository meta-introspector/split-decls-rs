// Generated macro for impl_1403 (impl)
macro_rules! Depcrate_strimpl_1403 {
() => {
// Module: crate::str
// Provides: {"impl_1403"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl ToOwned for str { type Owned = String ; # [inline] fn to_owned (& self) -> String { unsafe { String :: from_utf8_unchecked (self . as_bytes () . to_owned ()) } } # [inline] fn clone_into (& self , target : & mut String) { target . clear () ; target . push_str (self) ; } }
};
}
