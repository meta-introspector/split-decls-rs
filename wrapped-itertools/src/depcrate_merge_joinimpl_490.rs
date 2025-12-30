// Generated macro for impl_490 (impl)
macro_rules! Depcrate_merge_joinimpl_490 {
() => {
// Module: crate::merge_join
// Provides: {"impl_490"}
// Dependencies: {}
impl < I , J , F > fmt :: Debug for MergeBy < I , J , F > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug , J : Iterator + fmt :: Debug , J :: Item : fmt :: Debug , { debug_fmt_fields ! (MergeBy , left , right) ; }
};
}
