// Generated macro for check_ser_any_number (function)
macro_rules! Depcrate_ser_testscheck_ser_any_number {
() => {
// Module: crate::ser::tests
// Provides: {"check_ser_any_number"}
// Dependencies: {}
fn check_ser_any_number < T : Copy + Into < Number > + core :: fmt :: Display + serde :: Serialize > (n : T) { let mut fmt = format ! ("{}" , n) ; if ! fmt . contains ('.') && core :: any :: type_name :: < T > () . contains ('f') { fmt . push_str (".0") ; } check_to_string_writer (& n . into () , & fmt , & fmt) ; check_to_string_writer (& n , & fmt , & fmt) ; }
};
}
