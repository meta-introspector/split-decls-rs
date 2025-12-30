// Generated macro for impl_93 (impl)
macro_rules! Depcrate_iterableimpl_93 {
() => {
// Module: crate::iterable
// Provides: {"impl_93"}
// Dependencies: {}
impl < T > IIterable_Impl < T > for StockIterable_Impl < T > where T : RuntimeType , T :: Default : Clone , { fn First (& self) -> Result < IIterator < T > > { Ok (ComObject :: new (StockIterator { owner : self . to_object () , current : 0 . into () , }) . into_interface ()) } }
};
}
