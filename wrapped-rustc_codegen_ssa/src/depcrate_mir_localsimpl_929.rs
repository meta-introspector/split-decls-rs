// Generated macro for impl_929 (impl)
macro_rules! Depcrate_mir_localsimpl_929 {
() => {
// Module: crate::mir::locals
// Provides: {"impl_929"}
// Dependencies: {}
impl < 'tcx , V > Index < mir :: Local > for Locals < 'tcx , V > { type Output = LocalRef < 'tcx , V > ; # [inline] fn index (& self , index : mir :: Local) -> & LocalRef < 'tcx , V > { & self . values [index] } }
};
}
