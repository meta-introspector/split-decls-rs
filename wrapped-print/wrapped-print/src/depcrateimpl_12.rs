// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Source { fn stdin () -> Self { Source :: Stdin (io :: stdin ()) } async fn tcp (host : String , port : u16 , set_addr : bool) -> anyhow :: Result < Self > { match TcpStream :: connect ((host , port)) . await { Ok (stream) => Ok (Source :: Tcp (stream , set_addr)) , Err (e) => Err (anyhow ! (e)) , } } fn serial (path : PathBuf , baud : u32 , dtr : bool) -> anyhow :: Result < Self > { let mut ser = tokio_serial :: new (path . to_string_lossy () , baud) . open_native_async () ? ; ser . set_timeout (Duration :: from_millis (500)) ? ; if dtr { ser . write_data_terminal_ready (true) ? ; } Ok (Source :: Serial (ser)) } async fn set_rtt_addr (& mut self , elf_bytes : & [u8]) -> anyhow :: Result < () > { let Source :: Tcp (tcpstream , set_addr) = self else { return Ok (()) ; } ; if ! * set_addr { return Ok (()) ; } let elf = Elf :: parse (elf_bytes) ? ; let rtt_symbol = elf . syms . iter () . find (| sym | elf . strtab . get_at (sym . st_name) == Some ("_SEGGER_RTT")) . ok_or_else (| | anyhow ! ("Symbol '_SEGGER_RTT' not found in ELF file")) ? ; let cmd = format ! ("$$SEGGER_TELNET_ConfigStr=SetRTTAddr;{:#x}$$" , rtt_symbol . st_value) ; tcpstream . write_all (cmd . as_bytes ()) . await ? ; Ok (()) } async fn read (& mut self , buf : & mut [u8]) -> anyhow :: Result < (usize , bool) > { match self { Source :: Stdin (stdin) => { let n = stdin . read (buf) . await ? ; Ok ((n , n == 0)) } Source :: Tcp (tcpstream , ..) => Ok ((tcpstream . read (buf) . await ? , false)) , Source :: Serial (serial) => Ok ((serial . read (buf) . await ? , false)) , } } }
};
}
