macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
        RegionConstraintStorage!();
    };
}

macro_rules! RegionConstraintCollector {
    () => {
        deps!();
        pub struct RegionConstraintCollector < 'a , 'tcx > { storage : & 'a mut RegionConstraintStorage < 'tcx > , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > , }
    };
}

RegionConstraintCollector!()