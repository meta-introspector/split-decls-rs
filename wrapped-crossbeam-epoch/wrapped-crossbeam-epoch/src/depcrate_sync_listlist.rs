// Generated macro for List (struct)
macro_rules! Depcrate_sync_listList {
() => {
// Module: crate::sync::list
// Provides: {"List"}
// Dependencies: {}
# [doc = " A lock-free, intrusive linked list of type `T`."] # [derive (Debug)] pub (crate) struct List < T , C : IsElement < T > = T > { # [doc = " The head of the linked list."] head : Atomic < Entry > , # [doc = " The phantom data for using `T` and `C`."] _marker : PhantomData < (T , C) > , }
};
}
