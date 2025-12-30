// Generated macro for wndproc (function)
macro_rules! Depcratewndproc {
() => {
// Module: crate
// Provides: {"wndproc"}
// Dependencies: {}
extern "system" fn wndproc < S : DXSample > (window : HWND , message : u32 , wparam : WPARAM , lparam : LPARAM ,) -> LRESULT { match message { WM_CREATE => { unsafe { let create_struct : & CREATESTRUCTA = & * (lparam . 0 as * const CREATESTRUCTA) ; SetWindowLongPtrA (window , GWLP_USERDATA , create_struct . lpCreateParams as _) ; } LRESULT :: default () } WM_DESTROY => { unsafe { PostQuitMessage (0) } ; LRESULT :: default () } _ => { let user_data = unsafe { GetWindowLongPtrA (window , GWLP_USERDATA) } ; let sample = std :: ptr :: NonNull :: < S > :: new (user_data as _) ; let handled = sample . is_some_and (| mut s | sample_wndproc (unsafe { s . as_mut () } , message , wparam)) ; if handled { LRESULT :: default () } else { unsafe { DefWindowProcA (window , message , wparam , lparam) } } } } }
};
}
