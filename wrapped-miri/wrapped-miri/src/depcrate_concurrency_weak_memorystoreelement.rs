// Generated macro for StoreElement (struct)
macro_rules! Depcrate_concurrency_weak_memoryStoreElement {
() => {
// Module: crate::concurrency::weak_memory
// Provides: {"StoreElement"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] struct StoreElement { # [doc = " The identifier of the vector index, corresponding to a thread"] # [doc = " that performed the store."] store_index : VectorIdx , # [doc = " Whether this store is SC."] is_seqcst : bool , # [doc = " The timestamp of the storing thread when it performed the store"] timestamp : VTimestamp , # [doc = " The value of this store. `None` means uninitialized."] val : Option < Scalar > , # [doc = " Metadata about loads from this store element,"] # [doc = " behind a RefCell to keep load op take &self"] load_info : RefCell < LoadInfo > , }
};
}
