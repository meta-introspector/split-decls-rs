// Generated macro for SearchResult (enum)
macro_rules! Depcrate_collections_btree_searchSearchResult {
() => {
// Module: crate::collections::btree::search
// Provides: {"SearchResult"}
// Dependencies: {}
pub (super) enum SearchResult < BorrowType , K , V , FoundType , GoDownType > { Found (Handle < NodeRef < BorrowType , K , V , FoundType > , marker :: KV >) , GoDown (Handle < NodeRef < BorrowType , K , V , GoDownType > , marker :: Edge >) , }
};
}
