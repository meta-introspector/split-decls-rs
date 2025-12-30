// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl FileRuntime { fn new (expect : & Expect) -> FileRuntime { let path = to_abs_ws_path (Path :: new (expect . position . file)) ; let original_text = fs :: read_to_string (& path) . unwrap () ; let patchwork = Patchwork :: new (original_text . clone ()) ; FileRuntime { path , original_text , patchwork } } fn update (& mut self , expect : & Expect , actual : & str) { let loc = expect . locate (& self . original_text) ; let desired_indent = if expect . indent { Some (loc . line_indent) } else { None } ; let patch = format_patch (desired_indent , actual) ; self . patchwork . patch (loc . literal_range , & patch) ; fs :: write (& self . path , & self . patchwork . text) . unwrap () } }
};
}
