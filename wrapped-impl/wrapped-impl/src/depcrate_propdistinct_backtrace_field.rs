// Generated macro for distinct_backtrace_field (function)
macro_rules! Depcrate_propdistinct_backtrace_field {
() => {
// Module: crate::prop
// Provides: {"distinct_backtrace_field"}
// Dependencies: {}
fn distinct_backtrace_field < 'a , 'b > (backtrace_field : & 'a Field < 'b > , from_field : Option < & Field > ,) -> Option < & 'a Field < 'b > > { if from_field . map_or (false , | from_field | { from_field . member == backtrace_field . member }) { None } else { Some (backtrace_field) } }
};
}
