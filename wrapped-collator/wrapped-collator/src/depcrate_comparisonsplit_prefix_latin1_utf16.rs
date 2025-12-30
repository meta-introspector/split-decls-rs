// Generated macro for split_prefix_latin1_utf16 (function)
macro_rules! Depcrate_comparisonsplit_prefix_latin1_utf16 {
() => {
// Module: crate::comparison
// Provides: {"split_prefix_latin1_utf16"}
// Dependencies: {}
# [doc = " Finds the identical prefix of `left` containing Latin1"] # [doc = " and `right` containing potentially ill-formed UTF-16."] # [doc = ""] # [doc = " Returns the identical prefix, the part of `left` after the"] # [doc = " prefix, and the part of `right` after the prefix."] # [doc = ""] # [doc = " ✨ *Enabled with the `latin1` Cargo feature.*"] # [cfg (feature = "latin1")] fn split_prefix_latin1_utf16 < 'a , 'b > (left : & 'a [u8] , right : & 'b [u16] ,) -> (& 'a [u8] , & 'a [u8] , & 'b [u16]) { let i = left . iter () . zip (right . iter ()) . take_while (| (l , r) | u16 :: from (* * l) == * * r) . count () ; if let Some ((head , left_tail)) = left . split_at_checked (i) { if let Some (right_tail) = right . get (i ..) { return (head , left_tail , right_tail) ; } } (& [] , left , right) }
};
}
