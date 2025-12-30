// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl Master { fn open () -> Result < Self > { let master_fd = posix_openpt (OFlag :: O_RDWR) ? ; Ok (Self { fd : master_fd }) } fn grant_slave_access (& self) -> Result < () > { grantpt (& self . fd) } fn unlock_slave (& self) -> Result < () > { unlockpt (& self . fd) } fn get_slave_name (& self) -> Result < String > { get_slave_name (& self . fd) } # [cfg (not (target_os = "freebsd"))] fn get_slave_fd (& self) -> Result < RawFd > { let slave_name = self . get_slave_name () ? ; let slave_fd = open (slave_name . as_str () , OFlag :: O_RDWR | OFlag :: O_NOCTTY , Mode :: empty () ,) ? ; Ok (slave_fd) } # [cfg (target_os = "freebsd")] fn get_slave_fd (& self) -> Result < RawFd > { let slave_name = self . get_slave_name () ? ; let slave_fd = open (format ! ("/dev/{}" , slave_name . as_str ()) . as_str () , OFlag :: O_RDWR | OFlag :: O_NOCTTY , Mode :: empty () ,) ? ; Ok (slave_fd) } fn get_file_handle (& self) -> Result < File > { let fd = dup (self . as_raw_fd ()) ? ; let file = unsafe { File :: from_raw_fd (fd) } ; Ok (file) } }
};
}
