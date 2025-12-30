// Generated macro for impl_63 (impl)
macro_rules! Depcrate_baseimpl_63 {
() => {
// Module: crate::base
// Provides: {"impl_63"}
// Dependencies: {}
impl CFAllocator { # [inline] pub fn new (mut context : CFAllocatorContext) -> CFAllocator { unsafe { let allocator_ref = CFAllocatorCreate (kCFAllocatorDefault , & mut context) ; TCFType :: wrap_under_create_rule (allocator_ref) } } }
};
}
