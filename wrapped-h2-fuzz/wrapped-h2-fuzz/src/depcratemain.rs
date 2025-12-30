// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { env_logger :: init () ; let rt = tokio :: runtime :: Runtime :: new () . unwrap () ; loop { honggfuzz :: fuzz ! (| data : & [u8] | { eprintln ! ("{:?}" , rt . block_on (run (data))) ; }) ; } }
};
}
