// Generated macro for impl_235 (impl)
macro_rules! Depcrate_rt_lazy_staticimpl_235 {
() => {
// Module: crate::rt::lazy_static
// Provides: {"impl_235"}
// Dependencies: {}
impl StaticValue { pub (crate) fn new < T : 'static > (value : T) -> Self { Self { sync : Synchronize :: new () , v : Box :: new (value) , } } pub (crate) fn get < T : 'static > (& self) -> & T { self . v . downcast_ref :: < T > () . expect ("lazy value must downcast to expected type") } }
};
}
