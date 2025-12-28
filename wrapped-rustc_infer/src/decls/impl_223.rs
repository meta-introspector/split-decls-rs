macro_rules! deps {
    () => {
        TyVidEqKey!();
        TypeVariableStorage!();
        InferCtxtUndoLogs!();
        TypeVariableTable!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'tcx > TypeVariableStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> TypeVariableTable < 'a , 'tcx > { TypeVariableTable { storage : self , undo_log } } # [inline] pub (crate) fn eq_relations_ref (& self) -> & ut :: UnificationTableStorage < TyVidEqKey < 'tcx > > { & self . eq_relations } pub (super) fn finalize_rollback (& mut self) { debug_assert ! (self . values . len () >= self . eq_relations . len ()) ; self . values . truncate (self . eq_relations . len ()) ; } }
    };
}

impl_223!()