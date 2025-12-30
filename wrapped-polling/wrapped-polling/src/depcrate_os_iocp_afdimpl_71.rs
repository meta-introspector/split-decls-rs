// Generated macro for impl_71 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_71 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_71"}
// Dependencies: {}
impl NtdllImports { fn get () -> io :: Result < & 'static Self > { macro_rules ! s { ($ e : expr) => { { $ e as u16 } } ; } static NTDLL_NAME : & [u16] = & [s ! ('n') , s ! ('t') , s ! ('d') , s ! ('l') , s ! ('l') , s ! ('.') , s ! ('d') , s ! ('l') , s ! ('l') , s ! ('\0') ,] ; static NTDLL_IMPORTS : OnceLock < io :: Result < NtdllImports > > = OnceLock :: new () ; NTDLL_IMPORTS . get_or_init (| | unsafe { let ntdll = GetModuleHandleW (NTDLL_NAME . as_ptr () as * const _) ; if ntdll . is_null () { # [cfg (feature = "tracing")] tracing :: error ! ("Failed to load ntdll.dll") ; return Err (io :: Error :: last_os_error ()) ; } NtdllImports :: load (ntdll) }) . as_ref () . map_err (| e | io :: Error :: from (e . kind ())) } pub (super) fn force_load () -> io :: Result < () > { Self :: get () ? ; Ok (()) } }
};
}
