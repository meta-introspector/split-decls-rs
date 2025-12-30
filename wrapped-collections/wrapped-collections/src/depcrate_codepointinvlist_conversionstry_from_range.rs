// Generated macro for try_from_range (function)
macro_rules! Depcrate_codepointinvlist_conversionstry_from_range {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"try_from_range"}
// Dependencies: {}
fn try_from_range < 'data > (range : impl RangeBounds < char > ,) -> Result < CodePointInversionList < 'data > , RangeError > { let (from , till) = deconstruct_range (range) ; if from < till { let set = [PotentialCodePoint :: from_u24 (from) , PotentialCodePoint :: from_u24 (till) ,] ; let inv_list : ZeroVec < PotentialCodePoint > = ZeroVec :: alloc_from_slice (& set) ; # [expect (clippy :: unwrap_used)] Ok (CodePointInversionList :: try_from_inversion_list (inv_list) . unwrap ()) } else { Err (RangeError (from , till)) } }
};
}
