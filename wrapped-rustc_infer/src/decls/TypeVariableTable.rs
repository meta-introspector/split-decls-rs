macro_rules! deps {
    () => {
        TypeVariableStorage!();
        InferCtxtUndoLogs!();
    };
}

macro_rules! TypeVariableTable {
    () => {
        deps!();
        pub (crate) struct TypeVariableTable < 'a , 'tcx > { storage : & 'a mut TypeVariableStorage < 'tcx > , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > , }
    };
}

TypeVariableTable!();