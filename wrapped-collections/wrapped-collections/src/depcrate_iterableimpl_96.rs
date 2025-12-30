// Generated macro for impl_96 (impl)
macro_rules! Depcrate_iterableimpl_96 {
() => {
// Module: crate::iterable
// Provides: {"impl_96"}
// Dependencies: {}
impl < T > From < Vec < T :: Default > > for IIterable < T > where T : RuntimeType , T :: Default : Clone , { fn from (values : Vec < T :: Default >) -> Self { ComObject :: new (StockIterable { values }) . into_interface () } }
};
}
