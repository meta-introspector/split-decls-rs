macro_rules! deps {
    () => {
        InvalidAlignmentValue!();
        ArgParser!();
        AlignParser!();
        IncorrectReprFormatExpectInteger!();
        AcceptContext!();
        Stage!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl AlignParser { const PATH : & 'static [Symbol] = & [sym :: rustc_align] ; const TEMPLATE : AttributeTemplate = template ! (List : & ["<alignment in bytes>"]) ; fn parse < 'c , S : Stage > (& mut self , cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) { match args { ArgParser :: NoArgs | ArgParser :: NameValue (_) => { cx . expected_list (cx . attr_span) ; } ArgParser :: List (list) => { let Some (align) = list . single () else { cx . expected_single_argument (list . span) ; return ; } ; let Some (lit) = align . lit () else { cx . emit_err (session_diagnostics :: IncorrectReprFormatExpectInteger { span : align . span () , }) ; return ; } ; match parse_alignment (& lit . kind) { Ok (literal) => self . 0 = Ord :: max (self . 0 , Some ((literal , cx . attr_span))) , Err (message) => { cx . emit_err (session_diagnostics :: InvalidAlignmentValue { span : lit . span , error_part : message , }) ; } } } } } }
    };
}

impl_166!()