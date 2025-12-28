macro_rules! deps {
    () => {
        Kind!();
        Error!();
        Iter!();
    };
}

macro_rules! parse_line {
    () => {
        deps!();
        fn parse_line (line : & BStr , line_number : usize) -> Option < Result < (Kind , Iter < '_ > , usize) , Error > > { if line . is_empty () { return None ; } let (line , attrs) : (Cow < '_ , _ > , _) = if line . starts_with (b"\"") { let (unquoted , consumed) = match gix_quote :: ansi_c :: undo (line) { Ok (res) => res , Err (err) => return Some (Err (err . into ())) , } ; (unquoted , & line [consumed ..]) } else { line . find_byteset (BLANKS) . map (| pos | (line [.. pos] . as_bstr () . into () , line [pos ..] . as_bstr ())) . unwrap_or ((line . into () , [] . as_bstr ())) } ; let kind_res = match line . strip_prefix (b"[attr]") { Some (macro_name) => check_attr (macro_name . into ()) . map_err (| err | Error :: MacroName { line_number , macro_name : err . attribute , }) . map (| name | Kind :: Macro (name . to_owned ())) , None => { let pattern = gix_glob :: Pattern :: from_bytes (line . as_ref ()) ? ; if pattern . mode . contains (gix_glob :: pattern :: Mode :: NEGATIVE) { Err (Error :: PatternNegation { line : line . into_owned () , line_number , }) } else { Ok (Kind :: Pattern (pattern)) } } } ; let kind = match kind_res { Ok (kind) => kind , Err (err) => return Some (Err (err)) , } ; Ok ((kind , Iter :: new (attrs) , line_number)) . into () }
    };
}

parse_line!();