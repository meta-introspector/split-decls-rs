macro_rules! deps {
    () => {
        IllFormedAttributeInputLint!();
        AllowedTargets!();
        FinalizeContext!();
        AttributeParser!();
        MacroUseParser!();
        AcceptMapping!();
        AcceptContext!();
        Stage!();
        ArgParser!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < S : Stage > AttributeParser < S > for MacroUseParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: macro_use] , MACRO_USE_TEMPLATE , | group : & mut Self , cx : & mut AcceptContext < '_ , '_ , S > , args | { let span = cx . attr_span ; group . first_span . get_or_insert (span) ; match args { ArgParser :: NoArgs => { match group . state { MacroUseArgs :: UseAll => { let first_span = group . first_span . expect ("State is UseAll is some so this is not the first attribute" ,) ; cx . warn_unused_duplicate (first_span , span) ; } MacroUseArgs :: UseSpecific (_) => { group . state = MacroUseArgs :: UseAll ; group . first_span = Some (span) ; for specific_use in group . uses_attr_spans . drain (..) { cx . warn_unused_duplicate (span , specific_use) ; } } } } ArgParser :: List (list) => { if list . is_empty () { cx . warn_empty_attribute (list . span) ; return ; } match & mut group . state { MacroUseArgs :: UseAll => { let first_span = group . first_span . expect ("State is UseAll is some so this is not the first attribute" ,) ; cx . warn_unused_duplicate (first_span , span) ; } MacroUseArgs :: UseSpecific (arguments) => { group . uses_attr_spans . push (cx . attr_span) ; for item in list . mixed () { let Some (item) = item . meta_item () else { cx . expected_identifier (item . span ()) ; continue ; } ; if let Err (err_span) = item . args () . no_args () { cx . expected_no_args (err_span) ; continue ; } let Some (item) = item . path () . word () else { cx . expected_identifier (item . span ()) ; continue ; } ; arguments . push (item) ; } } } } ArgParser :: NameValue (_) => { let suggestions = MACRO_USE_TEMPLATE . suggestions (cx . attr_style , sym :: macro_use) ; cx . emit_err (IllFormedAttributeInputLint { num_suggestions : suggestions . len () , suggestions : DiagArgValue :: StrListSepByAnd (suggestions . into_iter () . map (| s | format ! ("`{s}`") . into ()) . collect () ,) , span , }) ; } } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = MACRO_USE_ALLOWED_TARGETS ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { Some (AttributeKind :: MacroUse { span : self . first_span ? , arguments : self . state }) } }
    };
}

impl_124!();