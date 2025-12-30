// Generated macro for split_prefix_u16 (function)
macro_rules! Depcrate_comparisonsplit_prefix_u16 {
() => {
// Module: crate::comparison
// Provides: {"split_prefix_u16"}
// Dependencies: {}
# [doc = " Finds the identical prefix of `left` and `right` containing"] # [doc = " potentially ill-formed UTF-16, while avoiding splitting a"] # [doc = " well-formed surrogate pair. In case of ill-formed"] # [doc = " UTF-16, the prefix is not guaranteed to be maximal."] # [doc = ""] # [doc = " Returns the identical prefix, the part of `left` after the"] # [doc = " prefix, and the part of `right` after the prefix."] fn split_prefix_u16 < 'a , 'b > (left : & 'a [u16] , right : & 'b [u16] ,) -> (& 'a [u16] , & 'a [u16] , & 'b [u16]) { let mut i = left . iter () . zip (right . iter ()) . take_while (| (l , r) | l == r) . count () ; if i != 0 { if let Some (& last) = left . get (i . wrapping_sub (1)) { if in_inclusive_range16 (last , 0xD800 , 0xDBFF) { i -= 1 ; } if let Some ((head , left_tail)) = left . split_at_checked (i) { if let Some (right_tail) = right . get (i ..) { return (head , left_tail , right_tail) ; } } } } (& [] , left , right) }
};
}
