macro_rules! deps {
    () => {
        HirFormatter!();
        HirDisplayError!();
    };
}

macro_rules! hir_fmt_generics {
    () => {
        deps!();
        fn hir_fmt_generics < 'db > (f : & mut HirFormatter < '_ , 'db > , parameters : & [GenericArg < 'db >] , generic_def : Option < hir_def :: GenericDefId > , self_ : Option < Ty < 'db > > ,) -> Result < () , HirDisplayError > { if parameters . is_empty () { return Ok (()) ; } let parameters_to_write = generic_args_sans_defaults (f , generic_def , parameters) ; if ! parameters_to_write . is_empty () { write ! (f , "<") ? ; hir_fmt_generic_arguments (f , parameters_to_write , self_) ? ; write ! (f , ">") ? ; } Ok (()) }
    };
}

hir_fmt_generics!()