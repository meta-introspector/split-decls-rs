// Generated macro for extend_with_struct_pat (function)
macro_rules! Depcrate_unnested_or_patternsextend_with_struct_pat {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"extend_with_struct_pat"}
// Dependencies: {}
# [doc = " Here we focusing on a record pattern `S { fp_0, ..., fp_n }`."] # [doc = " In particular, for a record pattern, the order in which the field patterns is irrelevant."] # [doc = " So when we fixate on some `ident_k: pat_k`, we try to find `ident_k` in the other pattern"] # [doc = " and check that all `fp_i` where `i ∈ ((0...n) \\ k)` between two patterns are equal."] fn extend_with_struct_pat (qself1 : Option < & ast :: QSelf > , path1 : & ast :: Path , fps1 : & mut [ast :: PatField] , rest1 : ast :: PatFieldsRest , start : usize , alternatives : & mut ThinVec < Pat > ,) -> bool { (0 .. fps1 . len ()) . any (| idx | { let pos_in_2 = Cell :: new (None) ; let tail_or = drain_matching (start , alternatives , | k | { matches ! (k , Struct (qself2 , path2 , fps2 , rest2) if rest1 == * rest2 && eq_maybe_qself (qself1 , qself2 . as_deref ()) && eq_path (path1 , path2) && fps1 . len () == fps2 . len () && fps1 . iter () . enumerate () . all (| (idx_1 , fp1) | { if idx_1 == idx { let pos = fps2 . iter () . position (| fp2 | { ! (fp1 . is_shorthand && fp2 . is_shorthand) && eq_id (fp1 . ident , fp2 . ident) }) ; pos_in_2 . set (pos) ; pos . is_some () } else { fps2 . iter () . any (| fp2 | eq_field_pat (fp1 , fp2)) } })) } , | k | always_pat ! (k , Struct (_ , _ , mut fps , _) => * fps . swap_remove (pos_in_2 . take () . unwrap ()) . pat) ,) ; extend_with_tail_or (& mut fps1 [idx] . pat , tail_or) }) }
};
}
