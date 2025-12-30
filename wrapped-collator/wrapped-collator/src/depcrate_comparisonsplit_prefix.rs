// Generated macro for split_prefix (function)
macro_rules! Depcrate_comparisonsplit_prefix {
() => {
// Module: crate::comparison
// Provides: {"split_prefix"}
// Dependencies: {}
# [doc = " Finds the identical prefix of `left` and `right` containing"] # [doc = " guaranteed well-format UTF-8."] # [doc = ""] # [doc = " Returns the identical prefix, the part of `left` after the"] # [doc = " prefix, and the part of `right` after the prefix."] fn split_prefix < 'a , 'b > (left : & 'a str , right : & 'b str) -> (& 'a str , & 'a str , & 'b str) { let left_bytes = left . as_bytes () ; let right_bytes = right . as_bytes () ; let mut i = left_bytes . iter () . zip (right_bytes . iter ()) . take_while (| (l , r) | l == r) . count () ; if i != 0 { loop { if let Some (left_first) = left_bytes . get (i) { if (left_first & 0b1100_0000) == 0b1000_0000 { i -= 1 ; continue ; } } break ; } if let Some ((head , left_tail)) = left . split_at_checked (i) { if let Some (right_tail) = right . get (i ..) { return (head , left_tail , right_tail) ; } } } ("" , left , right) }
};
}
