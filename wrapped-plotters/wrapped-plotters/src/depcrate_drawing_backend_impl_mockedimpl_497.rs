// Generated macro for impl_497 (impl)
macro_rules! Depcrate_drawing_backend_impl_mockedimpl_497 {
() => {
// Module: crate::drawing::backend_impl::mocked
// Provides: {"impl_497"}
// Dependencies: {}
impl MockedBackend { pub fn new (width : u32 , height : u32) -> Self { MockedBackend { height , width , init_count : 0 , draw_count : 0 , num_draw_pixel_call : 0 , num_draw_line_call : 0 , num_draw_rect_call : 0 , num_draw_circle_call : 0 , num_draw_text_call : 0 , num_draw_path_call : 0 , num_fill_polygon_call : 0 , check_draw_pixel : vec ! [] . into () , check_draw_line : vec ! [] . into () , check_draw_rect : vec ! [] . into () , check_draw_path : vec ! [] . into () , check_draw_circle : vec ! [] . into () , check_draw_text : vec ! [] . into () , check_fill_polygon : vec ! [] . into () , drop_check : None , } } def_set_checker_func ! (check_draw_pixel , RGBAColor , BackendCoord) ; def_set_checker_func ! (check_draw_line , RGBAColor , u32 , BackendCoord , BackendCoord) ; def_set_checker_func ! (check_draw_rect , RGBAColor , u32 , bool , BackendCoord , BackendCoord) ; def_set_checker_func ! (check_draw_path , RGBAColor , u32 , Vec < BackendCoord >) ; def_set_checker_func ! (check_draw_circle , RGBAColor , u32 , bool , BackendCoord , u32) ; def_set_checker_func ! (check_draw_text , RGBAColor , & str , f64 , BackendCoord , & str) ; def_set_checker_func ! (drop_check , & Self) ; def_set_checker_func ! (check_fill_polygon , RGBAColor , Vec < BackendCoord >) ; fn check_before_draw (& mut self) { self . draw_count += 1 ; } }
};
}
