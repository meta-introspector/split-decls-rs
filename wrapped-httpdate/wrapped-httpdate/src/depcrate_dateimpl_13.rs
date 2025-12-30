// Generated macro for impl_13 (impl)
macro_rules! Depcrate_dateimpl_13 {
() => {
// Module: crate::date
// Provides: {"impl_13"}
// Dependencies: {}
impl HttpDate { fn is_valid (& self) -> bool { self . sec < 60 && self . min < 60 && self . hour < 24 && self . day > 0 && self . day < 32 && self . mon > 0 && self . mon <= 12 && self . year >= 1970 && self . year <= 9999 && & HttpDate :: from (SystemTime :: from (* self)) == self } }
};
}
