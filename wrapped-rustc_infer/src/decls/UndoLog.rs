macro_rules! deps {
    () => {
        ProjectionCacheEntry!();
        ProjectionCacheKey!();
    };
}

macro_rules! UndoLog {
    () => {
        deps!();
        pub (crate) type UndoLog < 'tcx > = snapshot_map :: UndoLog < ProjectionCacheKey < 'tcx > , ProjectionCacheEntry < 'tcx > > ;
    };
}

UndoLog!();