// Generated macro for impl_222 (impl)
macro_rules! Depcrate___macros_extern_class_checksimpl_222 {
() => {
// Module: crate::__macros::extern_class::checks
// Provides: {"impl_222"}
// Dependencies: {}
# [doc = " Always allow setting `MainThreadOnly`."] impl < 'a , Cls > ValidThreadKind < dyn MainThreadOnly + 'a > for Cls where Self : ClassType < ThreadKind = dyn MainThreadOnly + 'a > , Self :: Super : ClassType , { }
};
}
