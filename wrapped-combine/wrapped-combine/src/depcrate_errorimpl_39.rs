// Generated macro for impl_39 (impl)
macro_rules! Depcrate_errorimpl_39 {
() => {
// Module: crate::error
// Provides: {"impl_39"}
// Dependencies: {}
impl < 's , T , R > ErrorInfo < 's , T , R > for Range < R > where R : Clone , { type Format = & 'static str ; fn into_info (& 's self) -> Info < T , R , Self :: Format > { Info :: Range (self . 0 . clone ()) } }
};
}
