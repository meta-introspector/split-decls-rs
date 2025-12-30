// Generated macro for join_into (function)
macro_rules! Depcrate_joinjoin_into {
() => {
// Module: crate::join
// Provides: {"join_into"}
// Dependencies: {}
# [doc = " Implements `join`. Note that `input1` must be a variable, but"] # [doc = " `input2` can be either a variable or a relation. This is necessary"] # [doc = " because relations have no \"recent\" tuples, so the fn would be a"] # [doc = " guaranteed no-op if both arguments were relations.  See also"] # [doc = " `join_into_relation`."] pub (crate) fn join_into < 'me , Key : Ord , Val1 : Ord , Val2 : Ord , Result : Ord > (input1 : & Variable < (Key , Val1) > , input2 : impl JoinInput < 'me , (Key , Val2) > , output : & Variable < Result > , mut logic : impl FnMut (& Key , & Val1 , & Val2) -> Result ,) { let mut results = Vec :: new () ; let recent1 = input1 . recent () ; let recent2 = input2 . recent () ; { let mut closure = | k : & Key , v1 : & Val1 , v2 : & Val2 | results . push (logic (k , v1 , v2)) ; for batch2 in input2 . stable () . iter () { join_helper (& recent1 , & batch2 , & mut closure) ; } for batch1 in input1 . stable () . iter () { join_helper (& batch1 , & recent2 , & mut closure) ; } join_helper (& recent1 , & recent2 , & mut closure) ; } output . insert (Relation :: from_vec (results)) ; }
};
}
