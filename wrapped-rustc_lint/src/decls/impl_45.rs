macro_rules! deps {
    () => {
        UnusedDocComment!();
        EarlyContext!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl EarlyLintPass for UnusedDocComment { fn check_stmt (& mut self , cx : & EarlyContext < '_ > , stmt : & ast :: Stmt) { let kind = match stmt . kind { ast :: StmtKind :: Let (..) => "statements" , ast :: StmtKind :: Item (..) => return , ast :: StmtKind :: Empty | ast :: StmtKind :: Semi (_) | ast :: StmtKind :: Expr (_) | ast :: StmtKind :: MacCall (_) => return , } ; warn_if_doc (cx , stmt . span , kind , stmt . kind . attrs ()) ; } fn check_arm (& mut self , cx : & EarlyContext < '_ > , arm : & ast :: Arm) { if let Some (body) = & arm . body { let arm_span = arm . pat . span . with_hi (body . span . hi ()) ; warn_if_doc (cx , arm_span , "match arms" , & arm . attrs) ; } } fn check_pat (& mut self , cx : & EarlyContext < '_ > , pat : & ast :: Pat) { if let ast :: PatKind :: Struct (_ , _ , fields , _) = & pat . kind { for field in fields { warn_if_doc (cx , field . span , "pattern fields" , & field . attrs) ; } } } fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & ast :: Expr) { warn_if_doc (cx , expr . span , "expressions" , & expr . attrs) ; if let ExprKind :: Struct (s) = & expr . kind { for field in & s . fields { warn_if_doc (cx , field . span , "expression fields" , & field . attrs) ; } } } fn check_generic_param (& mut self , cx : & EarlyContext < '_ > , param : & ast :: GenericParam) { warn_if_doc (cx , param . ident . span , "generic parameters" , & param . attrs) ; } fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & ast :: Block) { warn_if_doc (cx , block . span , "blocks" , block . attrs ()) ; } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { if let ast :: ItemKind :: ForeignMod (_) = item . kind { warn_if_doc (cx , item . span , "extern blocks" , & item . attrs) ; } } }
    };
}

impl_45!()