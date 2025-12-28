macro_rules! deps {
    () => {
        ProjectionCacheKey!();
        ProjectionCacheEntry!();
    };
}

macro_rules! UndoLog {
    () => {
        deps!();
        pub (crate) type UndoLog < 'tcx > = snapshot_map :: UndoLog < ProjectionCacheKey < 'tcx > , ProjectionCacheEntry < 'tcx > > ;
    };
}

UndoLog!()