macro_rules! parse {
    () => {
        pub fn parse < 'a > (sess : & 'a Session) -> ast :: Crate { let mut krate = sess . time ("parse_crate" , | | { let mut parser = unwrap_or_emit_fatal (match & sess . io . input { Input :: File (file) => new_parser_from_file (& sess . psess , file , StripTokens :: ShebangAndFrontmatter , None ,) , Input :: Str { input , name } => new_parser_from_source_str (& sess . psess , name . clone () , input . clone () , StripTokens :: ShebangAndFrontmatter ,) , }) ; parser . parse_crate_mod () }) . unwrap_or_else (| parse_error | { let guar : ErrorGuaranteed = parse_error . emit () ; guar . raise_fatal () ; }) ; rustc_builtin_macros :: cmdline_attrs :: inject (& mut krate , & sess . psess , & sess . opts . unstable_opts . crate_attr ,) ; krate }
    };
}

parse!()