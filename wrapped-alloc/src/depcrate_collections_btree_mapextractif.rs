// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_collections_btree_mapExtractIf {
() => {
// Module: crate::collections::btree::map
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " An iterator produced by calling `extract_if` on BTreeMap."] # [stable (feature = "btree_extract_if" , since = "1.91.0")] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct ExtractIf < 'a , K , V , R , F , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { pred : F , inner : ExtractIfInner < 'a , K , V , R > , # [doc = " The BTreeMap will outlive this IntoIter so we don't care about drop order for `alloc`."] alloc : A , }
};
}
