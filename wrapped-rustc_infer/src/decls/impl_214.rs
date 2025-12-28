macro_rules! deps {
    () => {
        TypeVariableStorage!();
        TyVidEqKey!();
        UndoLog!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < 'tcx > Rollback < sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) { self . eq_relations . reverse (undo) } }
    };
}

impl_214!()