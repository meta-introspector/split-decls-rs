macro_rules! deps {
    () => {
        Output!();
        Step!();
        Input!();
        TopEntryPoint!();
        Parser!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl TopEntryPoint { pub fn parse (& self , input : & Input , edition : Edition) -> Output { let _p = tracing :: info_span ! ("TopEntryPoint::parse" , ? self) . entered () ; let entry_point : fn (& '_ mut parser :: Parser < '_ >) = match self { TopEntryPoint :: SourceFile => grammar :: entry :: top :: source_file , TopEntryPoint :: MacroStmts => grammar :: entry :: top :: macro_stmts , TopEntryPoint :: MacroItems => grammar :: entry :: top :: macro_items , TopEntryPoint :: Pattern => grammar :: entry :: top :: pattern , TopEntryPoint :: Type => grammar :: entry :: top :: type_ , TopEntryPoint :: Expr => grammar :: entry :: top :: expr , TopEntryPoint :: MetaItem => grammar :: entry :: top :: meta_item , } ; let mut p = parser :: Parser :: new (input , edition) ; entry_point (& mut p) ; let events = p . finish () ; let res = event :: process (events) ; if cfg ! (debug_assertions) { let mut depth = 0 ; let mut first = true ; for step in res . iter () { assert ! (depth > 0 || first) ; first = false ; match step { Step :: Enter { .. } => depth += 1 , Step :: Exit => depth -= 1 , Step :: FloatSplit { ends_in_dot : has_pseudo_dot } => { depth -= 1 + ! has_pseudo_dot as usize } Step :: Token { .. } | Step :: Error { .. } => () , } } assert ! (! first , "no tree at all") ; assert_eq ! (depth , 0 , "unbalanced tree") ; } res } }
    };
}

impl_113!();