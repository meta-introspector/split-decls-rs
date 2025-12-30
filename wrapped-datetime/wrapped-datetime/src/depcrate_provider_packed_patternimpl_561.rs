// Generated macro for impl_561 (impl)
macro_rules! Depcrate_provider_packed_patternimpl_561 {
() => {
// Module: crate::provider::packed_pattern
// Provides: {"impl_561"}
// Dependencies: {}
impl VariantPatternIndex { # [cfg (feature = "datagen")] pub (super) fn from_header_with_shift (header : u32 , shift : u32) -> Self { match Self :: try_from_u32 ((header >> shift) & constants :: CHUNK_MASK) { Some (x) => x , None => { debug_assert ! (false , "unreachable") ; Self :: Inherit } } } fn try_from_u32 (u : u32) -> Option < Self > { match u { 0 => Some (Self :: Inherit) , 1 => Some (Self :: I0) , 2 => Some (Self :: I1) , 3 => Some (Self :: I2) , 4 => Some (Self :: I3) , 5 => Some (Self :: I4) , 6 => Some (Self :: I5) , 7 => Some (Self :: I6) , _ => None , } } pub (super) fn try_from_chunks_u32 (chunks : [u32 ; 6]) -> Option < [Self ; 6] > { let [c0 , c1 , c2 , c3 , c4 , c5] = chunks ; Some ([Self :: try_from_u32 (c0) ? , Self :: try_from_u32 (c1) ? , Self :: try_from_u32 (c2) ? , Self :: try_from_u32 (c3) ? , Self :: try_from_u32 (c4) ? , Self :: try_from_u32 (c5) ? ,]) } pub (super) fn to_chunks_u32 (chunks : [Self ; 6]) -> [u32 ; 6] { let [c0 , c1 , c2 , c3 , c4 , c5] = chunks ; [c0 as u32 , c1 as u32 , c2 as u32 , c3 as u32 , c4 as u32 , c5 as u32 ,] } }
};
}
