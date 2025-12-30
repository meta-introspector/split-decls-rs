// Generated macro for emit_html_mir (function)
macro_rules! Depcrate_polonius_dumpemit_html_mir {
() => {
// Module: crate::polonius::dump
// Provides: {"emit_html_mir"}
// Dependencies: {}
# [doc = " Emits the polonius MIR, as escaped HTML."] fn emit_html_mir < 'tcx > (dumper : & MirDumper < '_ , '_ , 'tcx > , body : & Body < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { let mut buffer = Vec :: new () ; dumper . dump_mir_to_writer (body , & mut buffer) ? ; let buffer = String :: from_utf8_lossy (& buffer) ; for ch in buffer . chars () { let escaped = match ch { '>' => "&gt;" , '<' => "&lt;" , '&' => "&amp;" , '\'' => "&#39;" , '"' => "&quot;" , _ => { write ! (out , "{}" , ch) ? ; continue ; } } ; write ! (out , "{}" , escaped) ? ; } Ok (()) }
};
}
