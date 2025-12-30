// Generated macro for impl_354 (impl)
macro_rules! Depcrate_common_content_rangeimpl_354 {
() => {
// Module: crate::common::content_range
// Provides: {"impl_354"}
// Dependencies: {}
impl ContentRange { # [doc = " Construct a new `Content-Range: bytes ..` header."] pub fn bytes (range : impl RangeBounds < u64 > , complete_length : impl Into < Option < u64 > > ,) -> Result < ContentRange , InvalidContentRange > { let complete_length = complete_length . into () ; let start = match range . start_bound () { Bound :: Included (& s) => s , Bound :: Excluded (& s) => s + 1 , Bound :: Unbounded => 0 , } ; let end = match range . end_bound () { Bound :: Included (& e) => e , Bound :: Excluded (& e) => e - 1 , Bound :: Unbounded => match complete_length { Some (max) => max - 1 , None => return Err (InvalidContentRange { _inner : () }) , } , } ; Ok (ContentRange { range : Some ((start , end)) , complete_length , }) } # [doc = " Create a new `ContentRange` stating the range could not be satisfied."] # [doc = ""] # [doc = " The passed argument is the complete length of the entity."] pub fn unsatisfied_bytes (complete_length : u64) -> Self { ContentRange { range : None , complete_length : Some (complete_length) , } } # [doc = " Get the byte range if satisified."] # [doc = ""] # [doc = " Note that these byte ranges are inclusive on both ends."] pub fn bytes_range (& self) -> Option < (u64 , u64) > { self . range } # [doc = " Get the bytes complete length if available."] pub fn bytes_len (& self) -> Option < u64 > { self . complete_length } }
};
}
