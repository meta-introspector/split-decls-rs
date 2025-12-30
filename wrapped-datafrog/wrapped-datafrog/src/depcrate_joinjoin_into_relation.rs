// Generated macro for join_into_relation (function)
macro_rules! Depcrate_joinjoin_into_relation {
() => {
// Module: crate::join
// Provides: {"join_into_relation"}
// Dependencies: {}
# [doc = " Join, but for two relations."] pub (crate) fn join_into_relation < 'me , Key : Ord , Val1 : Ord , Val2 : Ord , Result : Ord > (input1 : & Relation < (Key , Val1) > , input2 : & Relation < (Key , Val2) > , mut logic : impl FnMut (& Key , & Val1 , & Val2) -> Result ,) -> Relation < Result > { let mut results = Vec :: new () ; join_helper (& input1 . elements , & input2 . elements , | k , v1 , v2 | { results . push (logic (k , v1 , v2)) ; }) ; Relation :: from_vec (results) }
};
}
