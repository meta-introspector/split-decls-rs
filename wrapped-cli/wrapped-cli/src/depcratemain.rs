// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { env_logger :: init () ; let options = Options :: parse () ; let info = os_info :: get () ; if options . all || ! (options . type_ || options . os_version || options . bitness || options . architecture) { if options . type_ || options . os_version || options . bitness || options . architecture { warn ! ("--all supersedes all other options") ; } println ! ("OS information:\nType: {}\nVersion: {}\nBitness: {} \narchitecture:{}" , info . os_type () , info . version () , info . bitness () , info . architecture () . unwrap ()) ; } else { if options . type_ { println ! ("OS type: {}" , info . os_type ()) ; } if options . os_version { println ! ("OS version: {}" , info . version ()) ; } if options . bitness { println ! ("OS bitness: {}" , info . bitness ()) ; } if options . architecture { println ! ("OS architecture: {}" , info . architecture () . unwrap ()) ; } } }
};
}
