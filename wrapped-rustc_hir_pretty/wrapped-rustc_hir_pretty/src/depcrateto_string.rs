// Generated macro for to_string (function)
macro_rules! Depcrateto_string {
() => {
// Module: crate
// Provides: {"to_string"}
// Dependencies: {}
fn to_string < F > (ann : & dyn PpAnn , f : F) -> String where F : FnOnce (& mut State < '_ >) , { let mut printer = State { s : pp :: Printer :: new () , comments : None , attrs : & | _ | & [] , ann } ; f (& mut printer) ; printer . s . eof () }
};
}
