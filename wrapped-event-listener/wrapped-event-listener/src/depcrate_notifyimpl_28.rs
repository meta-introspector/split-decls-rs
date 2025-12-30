// Generated macro for impl_28 (impl)
macro_rules! Depcrate_notifyimpl_28 {
() => {
// Module: crate::notify
// Provides: {"impl_28"}
// Dependencies: {}
impl < T , F : TagProducer < Tag = T > > GenericNotify < F > { pub (crate) fn new (count : usize , additional : bool , tags : F) -> Self { Self { count , additional , tags , } } }
};
}
