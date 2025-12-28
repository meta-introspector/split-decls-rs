macro_rules! deps {
    () => {
        UndoLog!();
        ProjectionCacheStorage!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'tcx > Rollback < UndoLog < 'tcx > > for ProjectionCacheStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { self . map . reverse (undo) ; } }
    };
}

impl_298!()