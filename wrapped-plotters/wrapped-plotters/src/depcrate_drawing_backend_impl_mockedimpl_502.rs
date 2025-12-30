// Generated macro for impl_502 (impl)
macro_rules! Depcrate_drawing_backend_impl_mockedimpl_502 {
() => {
// Module: crate::drawing::backend_impl::mocked
// Provides: {"impl_502"}
// Dependencies: {}
impl Drop for MockedBackend { fn drop (& mut self) { if std :: thread :: panicking () { return ; } let mut temp = None ; std :: mem :: swap (& mut temp , & mut self . drop_check) ; if let Some (mut checker) = temp { checker (self) ; } } }
};
}
