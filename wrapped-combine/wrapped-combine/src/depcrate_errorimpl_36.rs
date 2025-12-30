// Generated macro for impl_36 (impl)
macro_rules! Depcrate_errorimpl_36 {
() => {
// Module: crate::error
// Provides: {"impl_36"}
// Dependencies: {}
impl < 's , T , R > ErrorInfo < 's , T , R > for Token < T > where T : Clone , { type Format = & 'static str ; fn into_info (& 's self) -> Info < T , R , Self :: Format > { Info :: Token (self . 0 . clone ()) } }
};
}
