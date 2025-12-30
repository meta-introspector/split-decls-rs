// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_collections_btree_setExtractIf {
() => {
// Module: crate::collections::btree::set
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " An iterator produced by calling `extract_if` on BTreeSet."] # [stable (feature = "btree_extract_if" , since = "1.91.0")] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct ExtractIf < 'a , T , R , F , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { pred : F , inner : super :: map :: ExtractIfInner < 'a , T , SetValZST , R > , # [doc = " The BTreeMap will outlive this IntoIter so we don't care about drop order for `alloc`."] alloc : A , }
};
}
