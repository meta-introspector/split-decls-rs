macro_rules! deps {
    () => {
        HirFormatter!();
        HirDisplayError!();
        Canonical!();
        HirDisplay!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Const < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . kind () { ConstKind :: Placeholder (_) => write ! (f , "<placeholder>") , ConstKind :: Bound (BoundVarIndexKind :: Bound (db) , bound_const) => { write ! (f , "?{}.{}" , db . as_u32 () , bound_const . var . as_u32 ()) } ConstKind :: Bound (BoundVarIndexKind :: Canonical , bound_const) => { write ! (f , "?c.{}" , bound_const . var . as_u32 ()) } ConstKind :: Infer (..) => write ! (f , "#c#") , ConstKind :: Param (param) => { let generics = generics (f . db , param . id . parent ()) ; let param_data = & generics [param . id . local_id ()] ; write ! (f , "{}" , param_data . name () . unwrap () . display (f . db , f . edition ())) ? ; Ok (()) } ConstKind :: Value (const_bytes) => render_const_scalar (f , & const_bytes . value . inner () . memory , & const_bytes . value . inner () . memory_map , const_bytes . ty ,) , ConstKind :: Unevaluated (unev) => { let c = match unev . def { SolverDefId :: ConstId (id) => GeneralConstId :: ConstId (id) , SolverDefId :: StaticId (id) => GeneralConstId :: StaticId (id) , _ => unreachable ! () , } ; write ! (f , "{}" , c . name (f . db)) ? ; hir_fmt_generics (f , unev . args . as_slice () , c . generic_def (f . db) , None) ? ; Ok (()) } ConstKind :: Error (..) => f . write_char ('_') , ConstKind :: Expr (..) => write ! (f , "<const-expr>") , } } }
    };
}

impl_500!()