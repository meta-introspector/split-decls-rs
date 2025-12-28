macro_rules! deps {
    () => {
        CfgEval!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl CfgEval < '_ > { fn configure < T : HasAttrs + HasTokens > (& mut self , node : T) -> Option < T > { self . 0 . configure (node) } fn configure_annotatable (mut self , annotatable : Annotatable) -> Annotatable { if ! has_cfg_or_cfg_attr (& annotatable) { return annotatable ; } let orig_tokens = annotatable . to_tokens () ; let mut parser = Parser :: new (& self . 0 . sess . psess , orig_tokens , None) ; parser . capture_cfg = true ; let res : PResult < '_ , Annotatable > = try { match annotatable { Annotatable :: Item (_) => { let item = parser . parse_item (ForceCollect :: Yes) ? . unwrap () ; Annotatable :: Item (self . flat_map_item (item) . pop () . unwrap ()) } Annotatable :: AssocItem (_ , ctxt) => { let item = parser . parse_trait_item (ForceCollect :: Yes) ? . unwrap () . unwrap () ; Annotatable :: AssocItem (self . flat_map_assoc_item (item , ctxt) . pop () . unwrap () , ctxt ,) } Annotatable :: ForeignItem (_) => { let item = parser . parse_foreign_item (ForceCollect :: Yes) ? . unwrap () . unwrap () ; Annotatable :: ForeignItem (self . flat_map_foreign_item (item) . pop () . unwrap ()) } Annotatable :: Stmt (_) => { let stmt = parser . parse_stmt_without_recovery (false , ForceCollect :: Yes , false) ? . unwrap () ; Annotatable :: Stmt (Box :: new (self . flat_map_stmt (stmt) . pop () . unwrap ())) } Annotatable :: Expr (_) => { let mut expr = parser . parse_expr_force_collect () ? ; self . visit_expr (& mut expr) ; Annotatable :: Expr (expr) } _ => unreachable ! () , } } ; match res { Ok (ann) => ann , Err (err) => { err . emit () ; annotatable } } } }
    };
}

impl_23!();