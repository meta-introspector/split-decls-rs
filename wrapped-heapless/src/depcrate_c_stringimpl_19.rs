// Generated macro for impl_19 (impl)
macro_rules! Depcrate_c_stringimpl_19 {
() => {
// Module: crate::c_string
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < const N : usize , LenT : LenType > Zeroize for CString < N , LenT > { fn zeroize (& mut self) { self . inner . zeroize () ; const { assert ! (N > 0) ; } unsafe { self . inner . push_unchecked (b'\0') } ; } }
};
}
