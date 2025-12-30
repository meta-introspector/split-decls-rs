// Generated macro for read (function)
macro_rules! Depcrate_streamread {
() => {
// Module: crate::stream
// Provides: {"read"}
// Dependencies: {}
fn read (fd : & AsyncFd < Arc < FdGuard > > , buffer : & mut [u8] , cx : & mut Context ,) -> Poll < io :: Result < usize > > { let mut guard = ready ! (fd . poll_read_ready (cx)) ? ; let result = guard . try_io (| _ | { let read = read_into_buffer (fd . as_raw_fd () , buffer) ; if read == - 1 { return Err (io :: Error :: last_os_error ()) ; } Ok (read as usize) }) ; match result { Ok (result) => Poll :: Ready (result) , Err (_would_block) => { cx . waker () . wake_by_ref () ; Poll :: Pending } } }
};
}
