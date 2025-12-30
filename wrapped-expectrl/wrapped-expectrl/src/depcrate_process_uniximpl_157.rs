// Generated macro for impl_157 (impl)
macro_rules! Depcrate_process_uniximpl_157 {
() => {
// Module: crate::process::unix
// Provides: {"impl_157"}
// Dependencies: {}
impl Healthcheck for UnixProcess { type Status = WaitStatus ; fn get_status (& self) -> Result < Self :: Status > { get_status (& self . proc) } fn is_alive (& self) -> Result < bool > { self . proc . is_alive () . map_err (to_io_error ("failed to determine if process is alive")) } }
};
}
