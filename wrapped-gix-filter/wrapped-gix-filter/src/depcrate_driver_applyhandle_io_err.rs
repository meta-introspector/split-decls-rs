// Generated macro for handle_io_err (function)
macro_rules! Depcrate_driver_applyhandle_io_err {
() => {
// Module: crate::driver::apply
// Provides: {"handle_io_err"}
// Dependencies: {}
pub (crate) fn handle_io_err (err : & std :: io :: Error , running : & mut HashMap < BString , process :: Client > , process : & BStr) { if matches ! (err . kind () , std :: io :: ErrorKind :: BrokenPipe | std :: io :: ErrorKind :: UnexpectedEof) { running . remove (process) . expect ("present or we wouldn't be here") ; } }
};
}
