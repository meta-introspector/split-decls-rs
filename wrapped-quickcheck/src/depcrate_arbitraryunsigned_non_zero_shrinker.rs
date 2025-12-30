// Generated macro for unsigned_non_zero_shrinker (macro)
macro_rules! Depcrate_arbitraryunsigned_non_zero_shrinker {
() => {
// Module: crate::arbitrary
// Provides: {"unsigned_non_zero_shrinker"}
// Dependencies: {}
macro_rules ! unsigned_non_zero_shrinker { ($ ty : tt) => { mod shrinker { pub struct UnsignedNonZeroShrinker { x : $ ty , i : $ ty , } impl UnsignedNonZeroShrinker { # [allow (clippy :: new_ret_no_self)] pub fn new (x : $ ty) -> Box < dyn Iterator < Item = $ ty >> { debug_assert ! (x > 0) ; if x == 1 { super :: empty_shrinker () } else { Box :: new (std :: iter :: once (1) . chain (UnsignedNonZeroShrinker { x , i : x / 2 } ,) ,) } } } impl Iterator for UnsignedNonZeroShrinker { type Item = $ ty ; fn next (& mut self) -> Option <$ ty > { if self . x - self . i < self . x { let result = Some (self . x - self . i) ; self . i /= 2 ; result } else { None } } } } } ; }
};
}
