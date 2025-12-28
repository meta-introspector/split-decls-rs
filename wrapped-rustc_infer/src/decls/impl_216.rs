macro_rules! deps {
    () => {
        TypeVariableStorage!();
        UndoLog!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < 'tcx > Rollback < UndoLog < 'tcx > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { match undo { UndoLog :: EqRelation (undo) => self . eq_relations . reverse (undo) , UndoLog :: SubRelation (undo) => self . sub_unification_table . reverse (undo) , } } }
    };
}

impl_216!()