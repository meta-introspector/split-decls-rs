macro_rules! deps {
    () => {
        AcceptContext!();
        IncorrectReprFormatPackedOneOrZeroArg!();
        IncorrectReprFormatAlignOneArg!();
        IncorrectReprFormatPackedExpectInteger!();
        IncorrectReprFormatExpectInteger!();
        Stage!();
        AlignKind!();
        MetaItemListParser!();
        InvalidReprGeneric!();
    };
}

macro_rules! parse_repr_align {
    () => {
        deps!();
        fn parse_repr_align < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , list : & MetaItemListParser < '_ > , param_span : Span , align_kind : AlignKind ,) -> Option < ReprAttr > { use AlignKind :: * ; let Some (align) = list . single () else { match align_kind { Packed => { cx . emit_err (session_diagnostics :: IncorrectReprFormatPackedOneOrZeroArg { span : param_span , }) ; } Align => { cx . emit_err (session_diagnostics :: IncorrectReprFormatAlignOneArg { span : param_span , }) ; } } return None ; } ; let Some (lit) = align . lit () else { match align_kind { Packed => { cx . emit_err (session_diagnostics :: IncorrectReprFormatPackedExpectInteger { span : align . span () , }) ; } Align => { cx . emit_err (session_diagnostics :: IncorrectReprFormatExpectInteger { span : align . span () , }) ; } } return None ; } ; match parse_alignment (& lit . kind) { Ok (literal) => Some (match align_kind { AlignKind :: Packed => ReprAttr :: ReprPacked (literal) , AlignKind :: Align => ReprAttr :: ReprAlign (literal) , }) , Err (message) => { cx . emit_err (session_diagnostics :: InvalidReprGeneric { span : lit . span , repr_arg : match align_kind { Packed => "packed" . to_string () , Align => "align" . to_string () , } , error_part : message , }) ; None } } }
    };
}

parse_repr_align!()