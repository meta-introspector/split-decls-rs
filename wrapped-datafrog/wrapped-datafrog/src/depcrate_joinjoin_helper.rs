// Generated macro for join_helper (function)
macro_rules! Depcrate_joinjoin_helper {
() => {
// Module: crate::join
// Provides: {"join_helper"}
// Dependencies: {}
fn join_helper < K : Ord , V1 , V2 > (mut slice1 : & [(K , V1)] , mut slice2 : & [(K , V2)] , mut result : impl FnMut (& K , & V1 , & V2) ,) { while ! slice1 . is_empty () && ! slice2 . is_empty () { use std :: cmp :: Ordering ; match slice1 [0] . 0 . cmp (& slice2 [0] . 0) { Ordering :: Less => { slice1 = gallop (slice1 , | x | x . 0 < slice2 [0] . 0) ; } Ordering :: Equal => { let count1 = slice1 . iter () . take_while (| x | x . 0 == slice1 [0] . 0) . count () ; let count2 = slice2 . iter () . take_while (| x | x . 0 == slice2 [0] . 0) . count () ; for index1 in 0 .. count1 { for s2 in slice2 [.. count2] . iter () { result (& slice1 [0] . 0 , & slice1 [index1] . 1 , & s2 . 1) ; } } slice1 = & slice1 [count1 ..] ; slice2 = & slice2 [count2 ..] ; } Ordering :: Greater => { slice2 = gallop (slice2 , | x | x . 0 < slice1 [0] . 0) ; } } } }
};
}
