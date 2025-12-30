// Generated macro for expand (function)
macro_rules! Depcrate_expand_macroexpand {
() => {
// Module: crate::expand_macro
// Provides: {"expand"}
// Dependencies: {}
fn expand (sema : & Semantics < '_ , RootDatabase > , expanded : SyntaxNode , error : & mut String , result_span_map : & mut SpanMap < SyntaxContext > , mut offset_in_original_node : i32 ,) -> SyntaxNode { let children = expanded . descendants () . filter_map (ast :: Item :: cast) ; let mut replacements = Vec :: new () ; for child in children { if let Some (new_node) = expand_macro_recur (sema , & child , error , result_span_map , TextSize :: new ((offset_in_original_node + (u32 :: from (child . syntax () . text_range () . start ()) as i32)) as u32 ,) ,) { offset_in_original_node = offset_in_original_node + (u32 :: from (new_node . text_range () . len ()) as i32) - (u32 :: from (child . syntax () . text_range () . len ()) as i32) ; if expanded == * child . syntax () { return new_node ; } replacements . push ((child , new_node)) ; } } replacements . into_iter () . rev () . for_each (| (old , new) | ted :: replace (old . syntax () , new)) ; expanded }
};
}
