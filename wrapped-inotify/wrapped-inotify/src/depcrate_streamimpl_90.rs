// Generated macro for impl_90 (impl)
macro_rules! Depcrate_streamimpl_90 {
() => {
// Module: crate::stream
// Provides: {"impl_90"}
// Dependencies: {}
impl < T > EventStream < T > where T : AsMut < [u8] > + AsRef < [u8] > , { # [doc = " Returns a new `EventStream` associated with the default reactor."] pub (crate) fn new (fd : Arc < FdGuard > , buffer : T) -> io :: Result < Self > { Ok (EventStream { fd : AsyncFd :: new (fd) ? , buffer , buffer_pos : 0 , unused_bytes : 0 , }) } # [doc = " Returns an instance of `Watches` to add and remove watches."] # [doc = " See [`Watches::add`] and [`Watches::remove`]."] pub fn watches (& self) -> Watches { Watches :: new (self . fd . get_ref () . clone ()) } # [doc = " Consumes the `EventStream` instance and returns an `Inotify` using the original"] # [doc = " file descriptor that was passed from `Inotify` to create the `EventStream`."] pub fn into_inotify (self) -> Inotify { Inotify :: from_file_descriptor (self . fd . into_inner ()) } }
};
}
