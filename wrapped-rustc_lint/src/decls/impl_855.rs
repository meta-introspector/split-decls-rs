macro_rules! deps {
    () => {
        EarlyContext!();
        UnusedDelimsCtx!();
        UnusedDelimLint!();
    };
}

macro_rules! impl_855 {
    () => {
        deps!();
        impl EarlyLintPass for UnusedBraces { fn check_stmt (& mut self , cx : & EarlyContext < '_ > , s : & ast :: Stmt) { < Self as UnusedDelimLint > :: check_stmt (self , cx , s) } # [inline] fn check_expr (& mut self , cx : & EarlyContext < '_ > , e : & ast :: Expr) { < Self as UnusedDelimLint > :: check_expr (self , cx , e) ; if let ExprKind :: Repeat (_ , ref anon_const) = e . kind { self . check_unused_delims_expr (cx , & anon_const . value , UnusedDelimsCtx :: AnonConst , false , None , None , false ,) ; } } fn check_generic_arg (& mut self , cx : & EarlyContext < '_ > , arg : & ast :: GenericArg) { if let ast :: GenericArg :: Const (ct) = arg { self . check_unused_delims_expr (cx , & ct . value , UnusedDelimsCtx :: AnonConst , false , None , None , false ,) ; } } fn check_variant (& mut self , cx : & EarlyContext < '_ > , v : & ast :: Variant) { if let Some (anon_const) = & v . disr_expr { self . check_unused_delims_expr (cx , & anon_const . value , UnusedDelimsCtx :: AnonConst , false , None , None , false ,) ; } } fn check_ty (& mut self , cx : & EarlyContext < '_ > , ty : & ast :: Ty) { match ty . kind { ast :: TyKind :: Array (_ , ref len) => { self . check_unused_delims_expr (cx , & len . value , UnusedDelimsCtx :: ArrayLenExpr , false , None , None , false ,) ; } ast :: TyKind :: Typeof (ref anon_const) => { self . check_unused_delims_expr (cx , & anon_const . value , UnusedDelimsCtx :: AnonConst , false , None , None , false ,) ; } _ => { } } } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { < Self as UnusedDelimLint > :: check_item (self , cx , item) } }
    };
}

impl_855!()