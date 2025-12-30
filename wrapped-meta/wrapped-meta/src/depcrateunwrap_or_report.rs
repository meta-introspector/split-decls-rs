// Generated macro for unwrap_or_report (function)
macro_rules! Depcrateunwrap_or_report {
() => {
// Module: crate
// Provides: {"unwrap_or_report"}
// Dependencies: {}
# [doc = " A helper that will unwrap the result or panic"] # [doc = " with the nicely formatted error message."] pub fn unwrap_or_report < T , E > (result : Result < T , E >) -> T where E : IntoIterator , E :: Item : Display , { result . unwrap_or_else (| e | { panic ! ("{}{}" , "grammar error\n\n" . to_owned () , & e . into_iter () . map (| error | format ! ("{}" , error)) . collect ::< Vec < _ >> () . join ("\n\n")) }) }
};
}
