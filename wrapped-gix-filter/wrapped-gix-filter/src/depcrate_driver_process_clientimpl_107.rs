// Generated macro for impl_107 (impl)
macro_rules! Depcrate_driver_process_clientimpl_107 {
() => {
// Module: crate::driver::process::client
// Provides: {"impl_107"}
// Dependencies: {}
impl Client { fn send_command_and_meta (& mut self , command : & str , meta : & mut dyn Iterator < Item = (& str , BString) > ,) -> Result < () , invoke :: Error > { self . input . write_all (format ! ("command={command}") . as_bytes ()) ? ; let mut buf = BString :: default () ; for (key , value) in meta { buf . clear () ; buf . push_str (key) ; buf . push (b'=') ; buf . push_str (& value) ; self . input . write_all (& buf) ? ; } encode :: flush_to_write (self . input . inner_mut ()) ? ; Ok (()) } }
};
}
