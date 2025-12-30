// Generated macro for read_qlog_from_file (function)
macro_rules! Depcrateread_qlog_from_file {
() => {
// Module: crate
// Provides: {"read_qlog_from_file"}
// Dependencies: {}
pub fn read_qlog_from_file < P : AsRef < Path > > (path : P ,) -> Result < Qlog , Box < dyn Error > > { let file = File :: open (path) ? ; let reader = BufReader :: new (file) ; let qlog = serde_json :: from_reader (reader) ? ; Ok (qlog) }
};
}
