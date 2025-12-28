macro_rules! deps {
    () => {
        Lifetime!();
        MetaVarKind!();
        Stmt!();
        Expr!();
        Pat!();
        Item!();
        Ty!();
        Path!();
        Block!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl fmt :: Display for MetaVarKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let sym = match self { MetaVarKind :: Item => sym :: item , MetaVarKind :: Block => sym :: block , MetaVarKind :: Stmt => sym :: stmt , MetaVarKind :: Pat (PatParam { inferred : true } | PatWithOr) => sym :: pat , MetaVarKind :: Pat (PatParam { inferred : false }) => sym :: pat_param , MetaVarKind :: Expr { kind : Expr2021 { inferred : true } | Expr , .. } => sym :: expr , MetaVarKind :: Expr { kind : Expr2021 { inferred : false } , .. } => sym :: expr_2021 , MetaVarKind :: Ty { .. } => sym :: ty , MetaVarKind :: Ident => sym :: ident , MetaVarKind :: Lifetime => sym :: lifetime , MetaVarKind :: Literal => sym :: literal , MetaVarKind :: Meta { .. } => sym :: meta , MetaVarKind :: Path => sym :: path , MetaVarKind :: Vis => sym :: vis , MetaVarKind :: TT => sym :: tt , } ; write ! (f , "{sym}") } }
    };
}

impl_395!()