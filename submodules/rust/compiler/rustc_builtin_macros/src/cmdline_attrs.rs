mkuse!{use rustc_ast :: { self as ast } ;}
mkuse!{use rustc_errors :: Diag ;}
mkuse!{use rustc_parse :: parser :: attr :: InnerAttrPolicy ;}
mkuse!{use rustc_parse :: { parse_in , source_str_to_stream } ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: FileName ;}

macro_rules! inject_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inject in module {}", module_path!());
    };
}

mkfn!{
    inject_introspect!();
    pub fn inject (krate : & mut ast :: Crate , psess : & ParseSess , attrs : & [String]) { for raw_attr in attrs { let source = format ! ("#![{raw_attr}]") ; let parse = | | -> Result < ast :: Attribute , Vec < Diag < '_ > > > { let tokens = source_str_to_stream (psess , FileName :: cli_crate_attr_source_code (raw_attr) , source , None ,) ? ; parse_in (psess , tokens , "<crate attribute>" , | p | { p . parse_attribute (InnerAttrPolicy :: Permitted) }) . map_err (| e | vec ! [e]) } ; let meta = match parse () { Ok (meta) => meta , Err (errs) => { for err in errs { err . emit () ; } continue ; } } ; krate . attrs . push (meta) ; } }
}