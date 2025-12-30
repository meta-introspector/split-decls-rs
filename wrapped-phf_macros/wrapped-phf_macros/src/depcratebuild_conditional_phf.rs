// Generated macro for build_conditional_phf (function)
macro_rules! Depcratebuild_conditional_phf {
() => {
// Module: crate
// Provides: {"build_conditional_phf"}
// Dependencies: {}
# [doc = " Generic function to build conditional PHF structures"] fn build_conditional_phf < F > (entries : & [Entry] , simple_builder : F , empty_structure : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream where F : Fn (& [Entry] , HashState) -> proc_macro2 :: TokenStream , { let unconditional : Vec < _ > = entries . iter () . filter (| e | e . attrs . is_empty ()) . collect () ; let conditional : Vec < _ > = entries . iter () . filter (| e | ! e . attrs . is_empty ()) . collect () ; if conditional . is_empty () { let state = phf_generator :: generate_hash (entries) ; return simple_builder (entries , state) ; } let mut variants = Vec :: new () ; let num_conditional = conditional . len () ; for mask in 0 .. (1 << num_conditional) { let mut variant_entries = unconditional . clone () ; for (i , & entry) in conditional . iter () . enumerate () { if (mask & (1 << i)) != 0 { variant_entries . push (entry) ; } } if variant_entries . is_empty () { continue ; } let entries_vec : Vec < Entry > = variant_entries . into_iter () . cloned () . collect () ; let state = phf_generator :: generate_hash (& entries_vec) ; let structure_tokens = simple_builder (& entries_vec , state) ; let conditions = build_cfg_conditions (mask , & conditional) ; let condition = combine_conditions (conditions) ; variants . push ((condition , structure_tokens)) ; } if variants . is_empty () { empty_structure } else { build_nested_conditional (variants) } }
};
}
