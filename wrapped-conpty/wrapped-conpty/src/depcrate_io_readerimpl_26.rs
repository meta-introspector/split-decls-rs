// Generated macro for impl_26 (impl)
macro_rules! Depcrate_io_readerimpl_26 {
() => {
// Module: crate::io::reader
// Provides: {"impl_26"}
// Dependencies: {}
impl PipeReader { # [doc = " Returns a new instance of PipeReader."] pub fn new (handle : HANDLE) -> Self { Self { handle , blocking : true , } } # [doc = " Sets a pipe to a non blocking mode."] # [doc = ""] # [doc = " It doesn't changes DUPed handles."] # [doc = ""] # [doc = " Mainly developed to not pile down libraries to include any windows API crate."] pub fn blocking (& mut self , on : bool) { self . blocking = on ; } # [doc = " Tries to clone a instance to a new one."] # [doc = " All cloned instances share the same underlaying data so"] # [doc = " Reading from one cloned pipe will affect an original pipe."] pub fn try_clone (& self) -> Result < Self , Error > { clone_handle (self . handle) . map_err (Into :: into) . map (Self :: new) } }
};
}
