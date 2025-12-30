// Generated macro for spawn_emulator (function)
macro_rules! Depcratespawn_emulator {
() => {
// Module: crate
// Provides: {"spawn_emulator"}
// Dependencies: {}
fn spawn_emulator (target : & str , server : & Path , tmpdir : & Path , rootfs : Option < PathBuf >) { let device_address = env :: var (REMOTE_ADDR_ENV) . unwrap_or (DEFAULT_ADDR . to_string ()) ; if env :: var (REMOTE_ADDR_ENV) . is_ok () { println ! ("Connecting to remote device {} ..." , device_address) ; } else if target . contains ("android") { start_android_emulator (server) ; } else { let rootfs = rootfs . as_ref () . expect ("need rootfs on non-android") ; start_qemu_emulator (target , rootfs , server , tmpdir) ; } loop { let dur = Duration :: from_millis (2000) ; if let Ok (mut client) = TcpStream :: connect (& device_address) { t ! (client . set_read_timeout (Some (dur))) ; t ! (client . set_write_timeout (Some (dur))) ; if client . write_all (b"ping") . is_ok () { let mut b = [0 ; 4] ; if client . read_exact (& mut b) . is_ok () { break ; } } } thread :: sleep (dur) ; } }
};
}
