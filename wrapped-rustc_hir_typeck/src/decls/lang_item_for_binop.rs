macro_rules! deps {
    () => {
        Op!();
    };
}

macro_rules! lang_item_for_binop {
    () => {
        deps!();
        fn lang_item_for_binop (tcx : TyCtxt < '_ > , op : Op) -> (Symbol , Option < hir :: def_id :: DefId >) { let lang = tcx . lang_items () ; match op { Op :: AssignOp (op) => match op . node { hir :: AssignOpKind :: AddAssign => (sym :: add_assign , lang . add_assign_trait ()) , hir :: AssignOpKind :: SubAssign => (sym :: sub_assign , lang . sub_assign_trait ()) , hir :: AssignOpKind :: MulAssign => (sym :: mul_assign , lang . mul_assign_trait ()) , hir :: AssignOpKind :: DivAssign => (sym :: div_assign , lang . div_assign_trait ()) , hir :: AssignOpKind :: RemAssign => (sym :: rem_assign , lang . rem_assign_trait ()) , hir :: AssignOpKind :: BitXorAssign => (sym :: bitxor_assign , lang . bitxor_assign_trait ()) , hir :: AssignOpKind :: BitAndAssign => (sym :: bitand_assign , lang . bitand_assign_trait ()) , hir :: AssignOpKind :: BitOrAssign => (sym :: bitor_assign , lang . bitor_assign_trait ()) , hir :: AssignOpKind :: ShlAssign => (sym :: shl_assign , lang . shl_assign_trait ()) , hir :: AssignOpKind :: ShrAssign => (sym :: shr_assign , lang . shr_assign_trait ()) , } , Op :: BinOp (op) => match op . node { hir :: BinOpKind :: Add => (sym :: add , lang . add_trait ()) , hir :: BinOpKind :: Sub => (sym :: sub , lang . sub_trait ()) , hir :: BinOpKind :: Mul => (sym :: mul , lang . mul_trait ()) , hir :: BinOpKind :: Div => (sym :: div , lang . div_trait ()) , hir :: BinOpKind :: Rem => (sym :: rem , lang . rem_trait ()) , hir :: BinOpKind :: BitXor => (sym :: bitxor , lang . bitxor_trait ()) , hir :: BinOpKind :: BitAnd => (sym :: bitand , lang . bitand_trait ()) , hir :: BinOpKind :: BitOr => (sym :: bitor , lang . bitor_trait ()) , hir :: BinOpKind :: Shl => (sym :: shl , lang . shl_trait ()) , hir :: BinOpKind :: Shr => (sym :: shr , lang . shr_trait ()) , hir :: BinOpKind :: Lt => (sym :: lt , lang . partial_ord_trait ()) , hir :: BinOpKind :: Le => (sym :: le , lang . partial_ord_trait ()) , hir :: BinOpKind :: Ge => (sym :: ge , lang . partial_ord_trait ()) , hir :: BinOpKind :: Gt => (sym :: gt , lang . partial_ord_trait ()) , hir :: BinOpKind :: Eq => (sym :: eq , lang . eq_trait ()) , hir :: BinOpKind :: Ne => (sym :: ne , lang . eq_trait ()) , hir :: BinOpKind :: And | hir :: BinOpKind :: Or => { bug ! ("&& and || are not overloadable") } } , } }
    };
}

lang_item_for_binop!()