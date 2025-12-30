// Generated macro for ident_swap_sugg (function)
macro_rules! Depcrate_suspicious_operation_groupingsident_swap_sugg {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"ident_swap_sugg"}
// Dependencies: {}
fn ident_swap_sugg (cx : & EarlyContext < '_ > , paired_identifiers : & FxHashSet < Ident > , binop : & BinaryOp < '_ > , location : IdentLocation , applicability : & mut Applicability ,) -> Option < String > { let left_ident = get_ident (binop . left , location) ? ; let right_ident = get_ident (binop . right , location) ? ; let sugg = match (paired_identifiers . contains (& left_ident) , paired_identifiers . contains (& right_ident) ,) { (true , true) | (false , false) => { * applicability = Applicability :: MaybeIncorrect ; let right_suggestion = suggestion_with_swapped_ident (cx , binop . right , location , left_ident , applicability) ? ; replace_right_sugg (cx , binop , & right_suggestion , applicability) } , (false , true) => { let right_suggestion = suggestion_with_swapped_ident (cx , binop . right , location , left_ident , applicability) ? ; replace_right_sugg (cx , binop , & right_suggestion , applicability) } , (true , false) => { let left_suggestion = suggestion_with_swapped_ident (cx , binop . left , location , right_ident , applicability) ? ; replace_left_sugg (cx , binop , & left_suggestion , applicability) } , } ; Some (sugg) }
};
}
