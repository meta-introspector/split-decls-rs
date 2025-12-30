// Generated macro for split_prefix_u8 (function)
macro_rules! Depcrate_comparisonsplit_prefix_u8 {
() => {
// Module: crate::comparison
// Provides: {"split_prefix_u8"}
// Dependencies: {}
# [doc = " Finds the identical prefix of `left` and `right` containing"] # [doc = " potentially ill-formed UTF-8, while avoiding splitting a UTF-8"] # [doc = " byte sequence. In case of ill-formed UTF-8, the prefix is"] # [doc = " not guaranteed to be maximal."] # [doc = ""] # [doc = " Returns the identical prefix, the part of `left` after the"] # [doc = " prefix, and the part of `right` after the prefix."] fn split_prefix_u8 < 'a , 'b > (left : & 'a [u8] , right : & 'b [u8]) -> (& 'a [u8] , & 'a [u8] , & 'b [u8]) { let mut i = left . iter () . zip (right . iter ()) . take_while (| (l , r) | l == r) . count () ; if i != 0 { if let Some (right_first) = right . get (i) { if (right_first & 0b1100_0000) == 0b1000_0000 { i -= 1 ; } } while i != 0 { if let Some (left_first) = left . get (i) { if (left_first & 0b1100_0000) == 0b1000_0000 { i -= 1 ; continue ; } } break ; } if let Some ((head , left_tail)) = left . split_at_checked (i) { if let Some (right_tail) = right . get (i ..) { return (head , left_tail , right_tail) ; } } } (& [] , left , right) }
};
}
