macro_rules! deps {
    () => {
        RegionConstraintStorage!();
        InferCtxtUndoLogs!();
        RegionConstraintCollector!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'tcx > RegionConstraintStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> RegionConstraintCollector < 'a , 'tcx > { RegionConstraintCollector { storage : self , undo_log } } }
    };
}

impl_139!();