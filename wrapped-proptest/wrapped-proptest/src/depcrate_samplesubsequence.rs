// Generated macro for subsequence (function)
macro_rules! Depcrate_samplesubsequence {
() => {
// Module: crate::sample
// Provides: {"subsequence"}
// Dependencies: {}
# [doc = " Sample subsequences whose size are within `size` from the given collection"] # [doc = " `values`."] # [doc = ""] # [doc = " A subsequence is a subset of the elements in a collection in the order they"] # [doc = " occur in that collection. The elements are not chosen to be contiguous."] # [doc = ""] # [doc = " This is roughly analogous to `rand::sample`, except that it guarantees that"] # [doc = " the order is preserved."] # [doc = ""] # [doc = " `values` may be a static slice or a `Vec`."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " Panics if the maximum size implied by `size` is larger than the size of"] # [doc = " `values`."] # [doc = ""] # [doc = " Panics if `size` is a zero-length range."] pub fn subsequence < T : Clone + 'static > (values : impl Into < Cow < 'static , [T] > > , size : impl Into < SizeRange > ,) -> Subsequence < T > { let values = values . into () ; let len = values . len () ; let size = size . into () ; size . assert_nonempty () ; assert ! (size . end_incl () <= len , "Maximum size of subsequence {} exceeds length of input {}" , size . end_incl () , len) ; Subsequence { values : Arc :: new (values) , bit_strategy : bits :: varsize :: sampled (size , 0 .. len) , } }
};
}
