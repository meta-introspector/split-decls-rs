// Generated macro for impl_168 (impl)
macro_rules! Depcrate_os_iocpimpl_168 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_168"}
// Dependencies: {}
impl WaitHandle { # [doc = " Wait for a waitable handle to become signaled."] fn new < F > (handle : RawHandle , callback : F , timeout : Option < Duration > , long_wait : bool ,) -> io :: Result < Self > where F : FnOnce () + Send + Sync + 'static , { struct AbortOnDrop ; impl Drop for AbortOnDrop { fn drop (& mut self) { std :: process :: abort () ; } } unsafe extern "system" fn wait_callback < F : FnOnce () + Send + Sync + 'static > (context : * mut c_void , _timer_fired : bool ,) { let _guard = AbortOnDrop ; let callback = Box :: from_raw (context as * mut F) ; callback () ; forget (_guard) ; } let mut wait_handle = MaybeUninit :: < RawHandle > :: uninit () ; let mut flags = WT_EXECUTEONLYONCE ; if long_wait { flags |= WT_EXECUTELONGFUNCTION ; } let res = unsafe { RegisterWaitForSingleObject (wait_handle . as_mut_ptr () . cast :: < _ > () , handle as _ , Some (wait_callback :: < F >) , Box :: into_raw (Box :: new (callback)) as _ , timeout . map_or (INFINITE , dur2timeout) , flags ,) } ; if res == 0 { return Err (io :: Error :: last_os_error ()) ; } let wait_handle = unsafe { wait_handle . assume_init () } ; Ok (Self (wait_handle)) } }
};
}
