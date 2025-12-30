// Generated macro for impl_4 (impl)
macro_rules! Depcrate_bindingsimpl_4 {
() => {
// Module: crate::bindings
// Provides: {"impl_4"}
// Dependencies: {}
impl Callback { pub fn new < F : Fn (i32) -> windows_core :: Result < i32 > + Send + 'static > (invoke : F) -> Self { let com = CallbackBox { vtable : & CallbackBox :: < F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke (& self , a : i32) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , a , & mut result__ ,) . map (| | result__) } } }
};
}
