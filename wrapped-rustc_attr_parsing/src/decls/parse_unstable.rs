macro_rules! deps {
    () => {
        AcceptContext!();
        ExpectsFeatures!();
        ExpectsFeatureList!();
        ArgParser!();
        Stage!();
    };
}

macro_rules! parse_unstable {
    () => {
        deps!();
        fn parse_unstable < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ > , symbol : Symbol ,) -> impl IntoIterator < Item = Symbol > { let mut res = Vec :: new () ; let Some (list) = args . list () else { cx . emit_err (session_diagnostics :: ExpectsFeatureList { span : cx . attr_span , name : symbol . to_ident_string () , }) ; return res ; } ; for param in list . mixed () { let param_span = param . span () ; if let Some (ident) = param . meta_item () . and_then (| i | i . path () . word ()) { res . push (ident . name) ; } else { cx . emit_err (session_diagnostics :: ExpectsFeatures { span : param_span , name : symbol . to_ident_string () , }) ; } } res }
    };
}

parse_unstable!()