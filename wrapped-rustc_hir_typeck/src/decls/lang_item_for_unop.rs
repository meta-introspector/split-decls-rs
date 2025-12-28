macro_rules! lang_item_for_unop {
    () => {
        fn lang_item_for_unop (tcx : TyCtxt < '_ > , op : hir :: UnOp) -> (Symbol , Option < hir :: def_id :: DefId >) { let lang = tcx . lang_items () ; match op { hir :: UnOp :: Not => (sym :: not , lang . not_trait ()) , hir :: UnOp :: Neg => (sym :: neg , lang . neg_trait ()) , hir :: UnOp :: Deref => bug ! ("Deref is not overloadable") , } }
    };
}

lang_item_for_unop!()