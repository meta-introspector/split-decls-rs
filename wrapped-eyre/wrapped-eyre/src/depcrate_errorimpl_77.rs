// Generated macro for impl_77 (impl)
macro_rules! Depcrate_errorimpl_77 {
() => {
// Module: crate::error
// Provides: {"impl_77"}
// Dependencies: {}
impl From < Report > for Box < dyn StdError + Send + Sync + 'static > { fn from (error : Report) -> Self { let outer = ManuallyDrop :: new (error) ; unsafe { (header (outer . inner . as_ref ()) . vtable . object_boxed) (outer . inner) } } }
};
}
