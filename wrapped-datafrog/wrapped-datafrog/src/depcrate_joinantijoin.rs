// Generated macro for antijoin (function)
macro_rules! Depcrate_joinantijoin {
() => {
// Module: crate::join
// Provides: {"antijoin"}
// Dependencies: {}
# [doc = " Moves all recent tuples from `input1` that are not present in `input2` into `output`."] pub (crate) fn antijoin < 'me , Key : Ord , Val : Ord , Result : Ord > (input1 : impl JoinInput < 'me , (Key , Val) > , input2 : & Relation < Key > , mut logic : impl FnMut (& Key , & Val) -> Result ,) -> Relation < Result > { let mut tuples2 = & input2 [..] ; let results = input1 . recent () . iter () . filter (| (ref key , _) | { tuples2 = gallop (tuples2 , | k | k < key) ; tuples2 . first () != Some (key) }) . map (| (ref key , ref val) | logic (key , val)) . collect :: < Vec < _ > > () ; Relation :: from_vec (results) }
};
}
