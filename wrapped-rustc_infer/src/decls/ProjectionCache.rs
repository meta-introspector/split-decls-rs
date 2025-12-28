macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
        ProjectionCacheKey!();
        ProjectionCacheEntry!();
    };
}

macro_rules! ProjectionCache {
    () => {
        deps!();
        # [doc = " The projection cache. Unlike the standard caches, this can include"] # [doc = " infcx-dependent type variables, therefore we have to roll the"] # [doc = " cache back each time we roll a snapshot back, to avoid assumptions"] # [doc = " on yet-unresolved inference variables. Types with placeholder"] # [doc = " regions also have to be removed when the respective snapshot ends."] # [doc = ""] # [doc = " Because of that, projection cache entries can be \"stranded\" and left"] # [doc = " inaccessible when type variables inside the key are resolved. We make no"] # [doc = " attempt to recover or remove \"stranded\" entries, but rather let them be"] # [doc = " (for the lifetime of the infcx)."] # [doc = ""] # [doc = " Entries in the projection cache might contain inference variables"] # [doc = " that will be resolved by obligations on the projection cache entry (e.g.,"] # [doc = " when a type parameter in the associated type is constrained through"] # [doc = " an \"RFC 447\" projection on the impl)."] # [doc = ""] # [doc = " When working with a fulfillment context, the derived obligations of each"] # [doc = " projection cache entry will be registered on the fulfillcx, so any users"] # [doc = " that can wait for a fulfillcx fixed point need not care about this. However,"] # [doc = " users that don't wait for a fixed point (e.g., trait evaluation) have to"] # [doc = " resolve the obligations themselves to make sure the projected result is"] # [doc = " ok and avoid issues like #43132."] # [doc = ""] # [doc = " If that is done, after evaluation the obligations, it is a good idea to"] # [doc = " call `ProjectionCache::complete` to make sure the obligations won't be"] # [doc = " re-evaluated and avoid an exponential worst-case."] pub struct ProjectionCache < 'a , 'tcx > { map : & 'a mut SnapshotMapStorage < ProjectionCacheKey < 'tcx > , ProjectionCacheEntry < 'tcx > > , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > , }
    };
}

ProjectionCache!();