// Generated macro for create_mocked_drawing_area (function)
macro_rules! Depcrate_drawing_backend_impl_mockedcreate_mocked_drawing_area {
() => {
// Module: crate::drawing::backend_impl::mocked
// Provides: {"create_mocked_drawing_area"}
// Dependencies: {}
pub fn create_mocked_drawing_area < F : FnOnce (& mut MockedBackend) > (width : u32 , height : u32 , setup : F ,) -> DrawingArea < MockedBackend , Shift > { let mut backend = MockedBackend :: new (width , height) ; setup (& mut backend) ; backend . into_drawing_area () }
};
}
