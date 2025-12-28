macro_rules! deps {
    () => {
        HirDisplayError!();
        HirFormatter!();
    };
}

macro_rules! hir_fmt_generic_arguments {
    () => {
        deps!();
        fn hir_fmt_generic_arguments < 'db > (f : & mut HirFormatter < '_ , 'db > , parameters : & [GenericArg < 'db >] , self_ : Option < Ty < 'db > > ,) -> Result < () , HirDisplayError > { let mut first = true ; let lifetime_offset = parameters . iter () . position (| arg | arg . region () . is_some ()) ; let (ty_or_const , lifetimes) = match lifetime_offset { Some (offset) => parameters . split_at (offset) , None => (parameters , & [] [..]) , } ; for generic_arg in lifetimes . iter () . chain (ty_or_const) { if ! mem :: take (& mut first) { write ! (f , ", ") ? ; } match self_ { self_ @ Some (_) if generic_arg . ty () == self_ => write ! (f , "Self") ? , _ => generic_arg . hir_fmt (f) ? , } } Ok (()) }
    };
}

hir_fmt_generic_arguments!();