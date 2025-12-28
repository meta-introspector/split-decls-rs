macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
        OpaqueTypeStorage!();
    };
}

macro_rules! OpaqueTypeTable {
    () => {
        deps!();
        pub struct OpaqueTypeTable < 'a , 'tcx > { storage : & 'a mut OpaqueTypeStorage < 'tcx > , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > , }
    };
}

OpaqueTypeTable!();