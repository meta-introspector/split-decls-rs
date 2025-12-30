// Generated macro for impl_223 (impl)
macro_rules! Depcrate___macros_extern_class_checksimpl_223 {
() => {
// Module: crate::__macros::extern_class::checks
// Provides: {"impl_223"}
// Dependencies: {}
# [doc = " But restrict `AnyThread` to only if the superclass also sets it."] impl < 'a , 'b , Cls > ValidThreadKind < dyn AnyThread + 'a > for Cls where Self : ClassType < ThreadKind = dyn AnyThread + 'a > , Self :: Super : ClassType < ThreadKind = dyn AnyThread + 'b > , { }
};
}
