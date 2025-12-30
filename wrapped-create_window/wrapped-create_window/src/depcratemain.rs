// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { let instance = GetModuleHandleA (None) ? ; let window_class = s ! ("window") ; let wc = WNDCLASSA { hCursor : LoadCursorW (None , IDC_ARROW) ? , hInstance : instance . into () , lpszClassName : window_class , style : CS_HREDRAW | CS_VREDRAW , lpfnWndProc : Some (wndproc) , .. Default :: default () } ; let atom = RegisterClassA (& wc) ; debug_assert ! (atom != 0) ; CreateWindowExA (WINDOW_EX_STYLE :: default () , window_class , s ! ("This is a sample window") , WS_OVERLAPPEDWINDOW | WS_VISIBLE , CW_USEDEFAULT , CW_USEDEFAULT , CW_USEDEFAULT , CW_USEDEFAULT , None , None , None , None ,) ? ; let mut message = MSG :: default () ; while GetMessageA (& mut message , None , 0 , 0) . into () { DispatchMessageA (& message) ; } Ok (()) } }
};
}
