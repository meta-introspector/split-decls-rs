// Generated macro for impl_260 (impl)
macro_rules! Depcrate_rt_threadimpl_260 {
() => {
// Module: crate::rt::thread
// Provides: {"impl_260"}
// Dependencies: {}
impl LocalValue { fn new < T : 'static > (value : T) -> Self { Self (Some (Box :: new (value))) } fn get < T : 'static > (& self) -> Result < & T , AccessError > { self . 0 . as_ref () . ok_or (AccessError { _private : () }) . map (| val | { val . downcast_ref :: < T > () . expect ("local value must downcast to expected type") }) } }
};
}
