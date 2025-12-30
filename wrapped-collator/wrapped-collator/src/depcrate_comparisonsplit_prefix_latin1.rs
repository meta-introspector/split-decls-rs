// Generated macro for split_prefix_latin1 (function)
macro_rules! Depcrate_comparisonsplit_prefix_latin1 {
() => {
// Module: crate::comparison
// Provides: {"split_prefix_latin1"}
// Dependencies: {}
# [doc = " Finds the identical prefix of `left` and `right` containing"] # [doc = " Latin1."] # [doc = ""] # [doc = " Returns the identical prefix, the part of `left` after the"] # [doc = " prefix, and the part of `right` after the prefix."] # [doc = ""] # [doc = " ✨ *Enabled with the `latin1` Cargo feature.*"] # [cfg (feature = "latin1")] fn split_prefix_latin1 < 'a , 'b > (left : & 'a [u8] , right : & 'b [u8]) -> (& 'a [u8] , & 'a [u8] , & 'b [u8]) { let i = left . iter () . zip (right . iter ()) . take_while (| (l , r) | l == r) . count () ; if let Some ((head , left_tail)) = left . split_at_checked (i) { if let Some (right_tail) = right . get (i ..) { return (head , left_tail , right_tail) ; } } (& [] , left , right) }
};
}
