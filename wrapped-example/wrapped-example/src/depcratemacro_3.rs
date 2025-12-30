// Generated macro for macro_3 (macro)
macro_rules! Depcratemacro_3 {
() => {
// Module: crate
// Provides: {"macro_3"}
// Dependencies: {}
lazy_static ! { static ref USERNAME : Regex = { println ! ("Compiling username regex...") ; Regex :: new ("^[a-z0-9_-]{3,16}$") . unwrap () } ; }
};
}
