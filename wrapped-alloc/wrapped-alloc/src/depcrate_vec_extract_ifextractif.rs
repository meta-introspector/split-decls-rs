// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_vec_extract_ifExtractIf {
() => {
// Module: crate::vec::extract_if
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " An iterator which uses a closure to determine if an element should be removed."] # [doc = ""] # [doc = " This struct is created by [`Vec::extract_if`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut v = vec![0, 1, 2];"] # [doc = " let iter: std::vec::ExtractIf<'_, _, _> = v.extract_if(.., |x| *x % 2 == 0);"] # [doc = " ```"] # [stable (feature = "extract_if" , since = "1.87.0")] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct ExtractIf < 'a , T , F , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { vec : & 'a mut Vec < T , A > , # [doc = " The index of the item that will be inspected by the next call to `next`."] idx : usize , # [doc = " Elements at and beyond this point will be retained. Must be equal or smaller than `old_len`."] end : usize , # [doc = " The number of items that have been drained (removed) thus far."] del : usize , # [doc = " The original length of `vec` prior to draining."] old_len : usize , # [doc = " The filter test predicate."] pred : F , }
};
}
