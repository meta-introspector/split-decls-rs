// Generated macro for start_read (function)
macro_rules! Depcrate_windowsstart_read {
() => {
// Module: crate::windows
// Provides: {"start_read"}
// Dependencies: {}
fn start_read (rd : & ReadData , event_handler : Arc < Mutex < dyn EventHandler > > , handle : HANDLE , action_tx : Sender < Action > ,) { let request = Box :: new (ReadDirectoryRequest { event_handler , handle , buffer : [0u8 ; BUF_SIZE as usize] , data : rd . clone () , action_tx , }) ; let flags = FILE_NOTIFY_CHANGE_FILE_NAME | FILE_NOTIFY_CHANGE_DIR_NAME | FILE_NOTIFY_CHANGE_ATTRIBUTES | FILE_NOTIFY_CHANGE_SIZE | FILE_NOTIFY_CHANGE_LAST_WRITE | FILE_NOTIFY_CHANGE_CREATION | FILE_NOTIFY_CHANGE_SECURITY ; let monitor_subdir = if request . data . file . is_none () && request . data . is_recursive { 1 } else { 0 } ; unsafe { let overlapped = alloc :: alloc_zeroed (alloc :: Layout :: new :: < OVERLAPPED > ()) as * mut OVERLAPPED ; let request = Box :: leak (request) ; (* overlapped) . hEvent = request as * mut _ as _ ; let ret = ReadDirectoryChangesW (handle , request . buffer . as_mut_ptr () as * mut c_void , BUF_SIZE , monitor_subdir , flags , & mut 0u32 as * mut u32 , overlapped , Some (handle_event) ,) ; if ret == 0 { let _overlapped = Box :: from_raw (overlapped) ; let request = Box :: from_raw (request) ; ReleaseSemaphore (request . data . complete_sem , 1 , ptr :: null_mut ()) ; } } }
};
}
