macro_rules! deps {
    () => {
        TypeVariableStorage!();
        UndoLog!();
        TyVidSubKey!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < 'tcx > Rollback < sv :: UndoLog < ut :: Delegate < TyVidSubKey > > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) { self . sub_unification_table . reverse (undo) } }
    };
}

impl_215!();