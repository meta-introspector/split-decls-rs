// Generated macro for impl_115 (impl)
macro_rules! Depcrate_rc_blockimpl_115 {
() => {
// Module: crate::rc_block
// Provides: {"impl_115"}
// Dependencies: {}
impl < F : ? Sized > Deref for RcBlock < F > { type Target = Block < F > ; # [inline] fn deref (& self) -> & Block < F > { unsafe { self . ptr . as_ref () } } }
};
}
