// Generated macro for Command (enum)
macro_rules! DepcrateCommand {
() => {
// Module: crate
// Provides: {"Command"}
// Dependencies: {}
# [derive (Subcommand , Clone)] enum Command { # [doc = " Read defmt frames from stdin (default)"] Stdin , # [doc = " Read defmt frames from a TCP server"] Tcp { # [doc = " Which host to connect to"] # [arg (long , env = "RTT_HOST" , default_value = "localhost")] host : String , # [doc = " Which port to connect to (uses the J-Link port by default)"] # [arg (long , env = "RTT_PORT" , default_value_t = 19021)] port : u16 , # [doc = " Tell Segger J-Link what the RTT address is"] # [arg (long)] set_addr : bool , } , # [doc = " Read defmt frames from a serial port"] Serial { # [arg (long , env = "SERIAL_PORT" , default_value = "/dev/ttyUSB0")] path : PathBuf , # [arg (long , env = "SERIAL_BAUD" , default_value_t = 115200)] baud : u32 , # [arg (long , env = "SERIAL_DTR" , default_value_t = false)] dtr : bool , } , }
};
}
