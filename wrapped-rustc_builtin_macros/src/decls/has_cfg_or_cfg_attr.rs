macro_rules! has_cfg_or_cfg_attr {
    () => {
        fn has_cfg_or_cfg_attr (annotatable : & Annotatable) -> bool { struct CfgFinder ; impl < 'ast > visit :: Visitor < 'ast > for CfgFinder { type Result = ControlFlow < () > ; fn visit_attribute (& mut self , attr : & 'ast Attribute) -> ControlFlow < () > { if attr . ident () . is_some_and (| ident | ident . name == sym :: cfg || ident . name == sym :: cfg_attr) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } } let res = match annotatable { Annotatable :: Item (item) => CfgFinder . visit_item (item) , Annotatable :: AssocItem (item , ctxt) => CfgFinder . visit_assoc_item (item , * ctxt) , Annotatable :: ForeignItem (item) => CfgFinder . visit_foreign_item (item) , Annotatable :: Stmt (stmt) => CfgFinder . visit_stmt (stmt) , Annotatable :: Expr (expr) => CfgFinder . visit_expr (expr) , _ => unreachable ! () , } ; res . is_break () }
    };
}

has_cfg_or_cfg_attr!();