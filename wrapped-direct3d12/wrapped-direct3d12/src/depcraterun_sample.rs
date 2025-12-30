// Generated macro for run_sample (function)
macro_rules! Depcraterun_sample {
() => {
// Module: crate
// Provides: {"run_sample"}
// Dependencies: {}
fn run_sample < S > () -> Result < () > where S : DXSample , { let instance = unsafe { GetModuleHandleA (None) ? } ; let wc = WNDCLASSEXA { cbSize : size_of :: < WNDCLASSEXA > () as u32 , style : CS_HREDRAW | CS_VREDRAW , lpfnWndProc : Some (wndproc :: < S >) , hInstance : instance . into () , hCursor : unsafe { LoadCursorW (None , IDC_ARROW) ? } , lpszClassName : s ! ("RustWindowClass") , .. Default :: default () } ; let command_line = build_command_line () ; let mut sample = S :: new (& command_line) ? ; let size = sample . window_size () ; let atom = unsafe { RegisterClassExA (& wc) } ; debug_assert_ne ! (atom , 0) ; let mut window_rect = RECT { left : 0 , top : 0 , right : size . 0 , bottom : size . 1 , } ; unsafe { AdjustWindowRect (& mut window_rect , WS_OVERLAPPEDWINDOW , false) ? } ; let mut title = sample . title () ; if command_line . use_warp_device { title . push_str (" (WARP)") ; } title . push ('\0') ; let hwnd = unsafe { CreateWindowExA (WINDOW_EX_STYLE :: default () , s ! ("RustWindowClass") , PCSTR (title . as_ptr ()) , WS_OVERLAPPEDWINDOW , CW_USEDEFAULT , CW_USEDEFAULT , window_rect . right - window_rect . left , window_rect . bottom - window_rect . top , None , None , None , Some (& mut sample as * mut _ as _) ,) } ? ; sample . bind_to_window (& hwnd) ? ; unsafe { _ = ShowWindow (hwnd , SW_SHOW) } ; loop { let mut message = MSG :: default () ; if unsafe { PeekMessageA (& mut message , None , 0 , 0 , PM_REMOVE) } . into () { unsafe { _ = TranslateMessage (& message) ; DispatchMessageA (& message) ; } if message . message == WM_QUIT { break ; } } } Ok (()) }
};
}
