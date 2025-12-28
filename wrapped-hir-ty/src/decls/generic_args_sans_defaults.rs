macro_rules! deps {
    () => {
        HirFormatter!();
    };
}

macro_rules! generic_args_sans_defaults {
    () => {
        deps!();
        fn generic_args_sans_defaults < 'ga , 'db > (f : & mut HirFormatter < '_ , 'db > , generic_def : Option < hir_def :: GenericDefId > , parameters : & 'ga [GenericArg < 'db >] ,) -> & 'ga [GenericArg < 'db >] { if f . display_kind . is_source_code () || f . omit_verbose_types () { match generic_def . map (| generic_def_id | f . db . generic_defaults (generic_def_id)) { None => parameters , Some (default_parameters) => { let should_show = | arg : GenericArg < 'db > , i : usize | match default_parameters . get (i) { None => true , Some (default_parameter) => { arg != default_parameter . instantiate (f . interner , & parameters [.. i]) } } ; let mut default_from = 0 ; for (i , & parameter) in parameters . iter () . enumerate () { if should_show (parameter , i) { default_from = i + 1 ; } } & parameters [0 .. default_from] } } } else { parameters } }
    };
}

generic_args_sans_defaults!();