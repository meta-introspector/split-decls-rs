// Generated macro for MockedBackend (struct)
macro_rules! Depcrate_drawing_backend_impl_mockedMockedBackend {
() => {
// Module: crate::drawing::backend_impl::mocked
// Provides: {"MockedBackend"}
// Dependencies: {}
pub struct MockedBackend { height : u32 , width : u32 , init_count : u32 , pub draw_count : u32 , pub num_draw_pixel_call : u32 , pub num_draw_line_call : u32 , pub num_draw_rect_call : u32 , pub num_draw_circle_call : u32 , pub num_draw_text_call : u32 , pub num_draw_path_call : u32 , pub num_fill_polygon_call : u32 , check_draw_pixel : VecDeque < Box < dyn FnMut (RGBAColor , BackendCoord) > > , check_draw_line : VecDeque < Box < dyn FnMut (RGBAColor , u32 , BackendCoord , BackendCoord) > > , check_draw_rect : VecDeque < Box < dyn FnMut (RGBAColor , u32 , bool , BackendCoord , BackendCoord) > > , check_draw_path : VecDeque < Box < dyn FnMut (RGBAColor , u32 , Vec < BackendCoord >) > > , check_draw_circle : VecDeque < Box < dyn FnMut (RGBAColor , u32 , bool , BackendCoord , u32) > > , check_draw_text : VecDeque < Box < dyn FnMut (RGBAColor , & str , f64 , BackendCoord , & str) > > , check_fill_polygon : VecDeque < Box < dyn FnMut (RGBAColor , Vec < BackendCoord >) > > , drop_check : Option < Box < dyn FnMut (& Self) > > , }
};
}
