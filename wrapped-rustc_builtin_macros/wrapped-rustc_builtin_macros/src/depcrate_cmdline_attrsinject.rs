// Generated macro for inject (function)
macro_rules! Depcrate_cmdline_attrsinject {
() => {
// Module: crate::cmdline_attrs
// Provides: {"inject"}
// Dependencies: {}
pub fn inject (krate : & mut ast :: Crate , psess : & ParseSess , attrs : & [String]) { for raw_attr in attrs { let source = format ! ("#![{raw_attr}]") ; let parse = | | -> Result < ast :: Attribute , Vec < Diag < '_ > > > { let tokens = source_str_to_stream (psess , FileName :: cli_crate_attr_source_code (raw_attr) , source , None ,) ? ; parse_in (psess , tokens , "<crate attribute>" , | p | { p . parse_attribute (InnerAttrPolicy :: Permitted) }) . map_err (| e | vec ! [e]) } ; let meta = match parse () { Ok (meta) => meta , Err (errs) => { for err in errs { err . emit () ; } continue ; } } ; krate . attrs . push (meta) ; } }
};
}
