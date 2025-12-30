// Generated macro for compare (macro)
macro_rules! Depcrate_comparisoncompare {
() => {
// Module: crate::comparison
// Provides: {"compare"}
// Dependencies: {}
macro_rules ! compare { ($ (# [$ meta : meta]) *, $ compare : ident , $ left_slice : ty , $ right_slice : ty , $ split_prefix : ident , $ left_to_iter : ident , $ right_to_iter : ident ,) => { $ (# [$ meta]) * pub fn $ compare (& self , left : &$ left_slice , right : &$ right_slice) -> Ordering { let (head , left_tail , right_tail) = $ split_prefix (left , right) ; if left_tail . is_empty () && right_tail . is_empty () { return Ordering :: Equal ; } let ret = self . compare_impl (left_tail .$ left_to_iter () , right_tail .$ right_to_iter () , head .$ left_to_iter () . rev ()) ; if self . options . strength () == Strength :: Identical && ret == Ordering :: Equal { return Decomposition :: new (left_tail .$ left_to_iter () , self . decompositions , self . tables) . map (| c | if c != MERGE_SEPARATOR { c as i32 } else { - 1i32 }) . cmp (Decomposition :: new (right_tail .$ right_to_iter () , self . decompositions , self . tables) . map (| c | if c != MERGE_SEPARATOR { c as i32 } else { - 1i32 }) ,) ; } ret } } }
};
}
