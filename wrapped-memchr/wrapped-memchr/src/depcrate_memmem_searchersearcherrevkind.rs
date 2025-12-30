// Generated macro for SearcherRevKind (enum)
macro_rules! Depcrate_memmem_searcherSearcherRevKind {
() => {
// Module: crate::memmem::searcher
// Provides: {"SearcherRevKind"}
// Dependencies: {}
# [doc = " The kind of the reverse searcher."] # [doc = ""] # [doc = " For the reverse case, we don't do any SIMD acceleration or prefilters."] # [doc = " There is no specific technical reason why we don't, but rather don't do it"] # [doc = " because it's not clear it's worth the extra code to do so. If you have a"] # [doc = " use case for it, please file an issue."] # [doc = ""] # [doc = " We also don't do the union trick as we do with the forward case and"] # [doc = " prefilters. Basically for the same reason we don't have prefilters or"] # [doc = " vector algorithms for reverse searching: it's not clear it's worth doing."] # [doc = " Please file an issue if you have a compelling use case for fast reverse"] # [doc = " substring search."] # [derive (Clone , Debug)] enum SearcherRevKind { Empty , OneByte { needle : u8 } , TwoWay { finder : twoway :: FinderRev } , }
};
}
