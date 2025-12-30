// Generated macro for impl_73 (impl)
macro_rules! Depcrate_errorimpl_73 {
() => {
// Module: crate::error
// Provides: {"impl_73"}
// Dependencies: {}
impl ErrorImpl < () > { pub (crate) fn error (this : RefPtr < '_ , Self >) -> & (dyn StdError + Send + Sync + 'static) { unsafe { (header (this) . vtable . object_ref) (this) } } pub (crate) fn error_mut (this : MutPtr < '_ , Self >) -> & mut (dyn StdError + Send + Sync + 'static) { unsafe { (header_mut (this) . vtable . object_mut) (this) } } pub (crate) fn chain (this : RefPtr < '_ , Self >) -> Chain < '_ > { Chain :: new (Self :: error (this)) } pub (crate) fn header (this : RefPtr < '_ , ErrorImpl >) -> & ErrorHeader { header (this) } }
};
}
