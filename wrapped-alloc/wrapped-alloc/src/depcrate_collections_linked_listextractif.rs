// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_collections_linked_listExtractIf {
() => {
// Module: crate::collections::linked_list
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " An iterator produced by calling `extract_if` on LinkedList."] # [stable (feature = "extract_if" , since = "1.87.0")] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct ExtractIf < 'a , T : 'a , F : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { list : & 'a mut LinkedList < T , A > , it : Option < NonNull < Node < T > > > , pred : F , idx : usize , old_len : usize , }
};
}
