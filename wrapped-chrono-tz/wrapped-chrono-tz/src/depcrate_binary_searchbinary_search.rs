// Generated macro for binary_search (function)
macro_rules! Depcrate_binary_searchbinary_search {
() => {
// Module: crate::binary_search
// Provides: {"binary_search"}
// Dependencies: {}
# [doc = " An implementation of binary search on indices only"] # [doc = " that does not require slices to be constructed. Mirrors"] # [doc = " the semantics of binary_search_by in the standard library."] pub fn binary_search < F > (mut start : usize , mut end : usize , mut f : F) -> Result < usize , usize > where F : FnMut (usize) -> Ordering , { loop { let mid = start + (end - start) / 2 ; if mid == end { return Err (start) ; } match f (mid) { Ordering :: Less => start = mid + 1 , Ordering :: Greater => end = mid , Ordering :: Equal => return Ok (mid) , } } }
};
}
