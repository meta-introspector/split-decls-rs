// Generated macro for wndproc (function)
macro_rules! Depcratewndproc {
() => {
// Module: crate
// Provides: {"wndproc"}
// Dependencies: {}
extern "system" fn wndproc (window : HWND , message : u32 , wparam : WPARAM , lparam : LPARAM) -> LRESULT { unsafe { match message { WM_PAINT => { println ! ("WM_PAINT") ; _ = ValidateRect (Some (window) , None) ; LRESULT (0) } WM_DESTROY => { println ! ("WM_DESTROY") ; PostQuitMessage (0) ; LRESULT (0) } _ => DefWindowProcA (window , message , wparam , lparam) , } } }
};
}
