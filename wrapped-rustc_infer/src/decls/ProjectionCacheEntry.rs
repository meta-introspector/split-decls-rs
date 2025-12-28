macro_rules! deps {
    () => {
        NormalizedTerm!();
    };
}

macro_rules! ProjectionCacheEntry {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub enum ProjectionCacheEntry < 'tcx > { InProgress , Ambiguous , Recur , Error , NormalizedTerm { ty : NormalizedTerm < 'tcx > , # [doc = " If we were able to successfully evaluate the corresponding cache"] # [doc = " entry key during predicate evaluation, then this field stores the"] # [doc = " final result obtained from evaluating all of the projection"] # [doc = " sub-obligations. During evaluation, we will skip evaluating the"] # [doc = " cached sub-obligations in `ty` if this field is set. Evaluation"] # [doc = " only cares about the final result, so we don't care about any"] # [doc = " region constraint side-effects produced by evaluating the"] # [doc = " sub-obligations."] # [doc = ""] # [doc = " Additionally, we will clear out the sub-obligations entirely if we"] # [doc = " ever evaluate the cache entry (along with all its sub obligations)"] # [doc = " to `EvaluatedToOk`. This affects all users of the cache, not just"] # [doc = " evaluation. Since a result of `EvaluatedToOk` means that there were"] # [doc = " no region obligations that need to be tracked, it's fine to forget"] # [doc = " about the sub-obligations - they don't provide any additional"] # [doc = " information. However, we do *not* discard any obligations when we"] # [doc = " see `EvaluatedToOkModuloRegions` - we don't know which"] # [doc = " sub-obligations may introduce region constraints, so we keep them"] # [doc = " all to be safe."] # [doc = ""] # [doc = " When we are not performing evaluation (e.g. in"] # [doc = " `FulfillmentContext`), we ignore this field, and always re-process"] # [doc = " the cached sub-obligations (which may have been cleared out - see"] # [doc = " the above paragraph). This ensures that we do not lose any regions"] # [doc = " constraints that arise from processing the sub-obligations."] complete : Option < EvaluationResult > , } , }
    };
}

ProjectionCacheEntry!();