// Generated macro for fanout (function)
macro_rules! Depcrate_index_encodefanout {
() => {
// Module: crate::index::encode
// Provides: {"fanout"}
// Dependencies: {}
pub (crate) fn fanout (iter : & mut dyn ExactSizeIterator < Item = u8 >) -> [u32 ; 256] { let mut fan_out = [0u32 ; 256] ; let entries_len = iter . len () as u32 ; let mut iter = iter . enumerate () ; let mut idx_and_entry = iter . next () ; let mut upper_bound = 0 ; for (offset_be , byte) in fan_out . iter_mut () . zip (0u8 ..= 255) { * offset_be = match idx_and_entry . as_ref () { Some ((_idx , first_byte)) => match first_byte . cmp (& byte) { Ordering :: Less => unreachable ! ("ids should be ordered, and we make sure to keep ahead with them") , Ordering :: Greater => upper_bound , Ordering :: Equal => { if byte == 255 { entries_len } else { idx_and_entry = iter . find (| (_ , first_byte) | * first_byte != byte) ; upper_bound = idx_and_entry . as_ref () . map_or (entries_len , | (idx , _) | * idx as u32) ; upper_bound } } } , None => entries_len , } ; } fan_out }
};
}
