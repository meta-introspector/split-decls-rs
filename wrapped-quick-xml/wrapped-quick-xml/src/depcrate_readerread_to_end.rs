// Generated macro for read_to_end (macro)
macro_rules! Depcrate_readerread_to_end {
() => {
// Module: crate::reader
// Provides: {"read_to_end"}
// Dependencies: {}
# [doc = " Generalization of `read_to_end` method for buffered and borrowed readers"] macro_rules ! read_to_end { ($ self : expr , $ end : expr , $ buf : expr , $ read_event : ident , $ clear : block $ (, $ await : ident) ?) => { { let config = $ self . config_mut () ; let trim = config . trim_text_start ; config . trim_text_start = false ; let start = $ self . buffer_position () ; let mut depth = 0 ; loop { $ clear let end = $ self . buffer_position () ; match $ self .$ read_event ($ buf) $ (.$ await) ? { Err (e) => { $ self . config_mut () . trim_text_start = trim ; return Err (e) ; } Ok (Event :: Start (e)) if e . name () == $ end => depth += 1 , Ok (Event :: End (e)) if e . name () == $ end => { if depth == 0 { $ self . config_mut () . trim_text_start = trim ; break start .. end ; } depth -= 1 ; } Ok (Event :: Eof) => { $ self . config_mut () . trim_text_start = trim ; return Err (Error :: missed_end ($ end , $ self . decoder ())) ; } _ => () , } } } } ; }
};
}
