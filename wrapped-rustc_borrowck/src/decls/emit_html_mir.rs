macro_rules! emit_html_mir {
    () => {
        # [doc = " Emits the polonius MIR, as escaped HTML."] fn emit_html_mir < 'tcx > (dumper : & MirDumper < '_ , '_ , 'tcx > , body : & Body < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { let mut buffer = Vec :: new () ; dumper . dump_mir_to_writer (body , & mut buffer) ? ; let buffer = String :: from_utf8_lossy (& buffer) ; for ch in buffer . chars () { let escaped = match ch { '>' => "&gt;" , '<' => "&lt;" , '&' => "&amp;" , '\'' => "&#39;" , '"' => "&quot;" , _ => { write ! (out , "{}" , ch) ? ; continue ; } } ; write ! (out , "{}" , escaped) ? ; } Ok (()) }
    };
}

emit_html_mir!()