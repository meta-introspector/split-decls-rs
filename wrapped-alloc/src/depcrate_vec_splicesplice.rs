// Generated macro for Splice (struct)
macro_rules! Depcrate_vec_spliceSplice {
() => {
// Module: crate::vec::splice
// Provides: {"Splice"}
// Dependencies: {}
# [doc = " A splicing iterator for `Vec`."] # [doc = ""] # [doc = " This struct is created by [`Vec::splice()`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut v = vec![0, 1, 2];"] # [doc = " let new = [7, 8];"] # [doc = " let iter: std::vec::Splice<'_, _> = v.splice(1.., new);"] # [doc = " ```"] # [derive (Debug)] # [stable (feature = "vec_splice" , since = "1.21.0")] pub struct Splice < 'a , I : Iterator + 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + 'a = Global , > { pub (super) drain : Drain < 'a , I :: Item , A > , pub (super) replace_with : I , }
};
}
