macro_rules! deps {
    () => {
        FormatUnknownTraitSugg!();
        FormatUnknownTrait!();
    };
}

macro_rules! invalid_placeholder_type_error {
    () => {
        deps!();
        fn invalid_placeholder_type_error (ecx : & ExtCtxt < '_ > , ty : & str , ty_span : Option < Range < usize > > , fmt_span : Span ,) { let sp = ty_span . map (| sp | fmt_span . from_inner (InnerSpan :: new (sp . start , sp . end))) ; let suggs = if let Some (sp) = sp { [("" , "Display") , ("?" , "Debug") , ("e" , "LowerExp") , ("E" , "UpperExp") , ("o" , "Octal") , ("p" , "Pointer") , ("b" , "Binary") , ("x" , "LowerHex") , ("X" , "UpperHex") ,] . into_iter () . map (| (fmt , trait_name) | errors :: FormatUnknownTraitSugg { span : sp , fmt , trait_name }) . collect () } else { vec ! [] } ; ecx . dcx () . emit_err (errors :: FormatUnknownTrait { span : sp . unwrap_or (fmt_span) , ty , suggs }) ; }
    };
}

invalid_placeholder_type_error!()