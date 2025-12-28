macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
        ProjectionCache!();
        ProjectionCacheStorage!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < 'tcx > ProjectionCacheStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> ProjectionCache < 'a , 'tcx > { ProjectionCache { map : & mut self . map , undo_log } } }
    };
}

impl_296!();