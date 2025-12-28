macro_rules! deps {
    () => {
        HirDisplayError!();
        HirFormatter!();
    };
}

macro_rules! hir_fmt_tys {
    () => {
        deps!();
        fn hir_fmt_tys < 'db > (f : & mut HirFormatter < '_ , 'db > , tys : & [Ty < 'db >] , self_ : Option < Ty < 'db > > ,) -> Result < () , HirDisplayError > { let mut first = true ; for ty in tys { if ! mem :: take (& mut first) { write ! (f , ", ") ? ; } match self_ { Some (self_) if * ty == self_ => write ! (f , "Self") ? , _ => ty . hir_fmt (f) ? , } } Ok (()) }
    };
}

hir_fmt_tys!()