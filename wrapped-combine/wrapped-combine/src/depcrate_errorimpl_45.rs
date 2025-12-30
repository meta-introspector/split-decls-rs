// Generated macro for impl_45 (impl)
macro_rules! Depcrate_errorimpl_45 {
() => {
// Module: crate::error
// Provides: {"impl_45"}
// Dependencies: {}
impl < 's , T , R , F > ErrorInfo < 's , T , R > for Format < F > where F : fmt :: Display + 's , { type Format = & 's F ; fn into_info (& 's self) -> Info < T , R , Self :: Format > { Info :: Format (& self . 0) } }
};
}
