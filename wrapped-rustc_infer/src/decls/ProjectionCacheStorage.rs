macro_rules! deps {
    () => {
        ProjectionCacheKey!();
        ProjectionCacheEntry!();
    };
}

macro_rules! ProjectionCacheStorage {
    () => {
        deps!();
        # [derive (Clone , Default)] pub struct ProjectionCacheStorage < 'tcx > { map : SnapshotMapStorage < ProjectionCacheKey < 'tcx > , ProjectionCacheEntry < 'tcx > > , }
    };
}

ProjectionCacheStorage!()