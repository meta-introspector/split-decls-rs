// Generated macro for prim_int_impl (macro)
macro_rules! Depcrate_intprim_int_impl {
() => {
// Module: crate::int
// Provides: {"prim_int_impl"}
// Dependencies: {}
macro_rules ! prim_int_impl { ($ T : ty , $ S : ty , $ U : ty) => { impl PrimInt for $ T { # [inline] fn count_ones (self) -> u32 { <$ T >:: count_ones (self) } # [inline] fn count_zeros (self) -> u32 { <$ T >:: count_zeros (self) } # [inline] fn leading_ones (self) -> u32 { <$ T >:: leading_ones (self) } # [inline] fn leading_zeros (self) -> u32 { <$ T >:: leading_zeros (self) } # [inline] fn trailing_ones (self) -> u32 { <$ T >:: trailing_ones (self) } # [inline] fn trailing_zeros (self) -> u32 { <$ T >:: trailing_zeros (self) } # [inline] fn rotate_left (self , n : u32) -> Self { <$ T >:: rotate_left (self , n) } # [inline] fn rotate_right (self , n : u32) -> Self { <$ T >:: rotate_right (self , n) } # [inline] fn signed_shl (self , n : u32) -> Self { ((self as $ S) << n) as $ T } # [inline] fn signed_shr (self , n : u32) -> Self { ((self as $ S) >> n) as $ T } # [inline] fn unsigned_shl (self , n : u32) -> Self { ((self as $ U) << n) as $ T } # [inline] fn unsigned_shr (self , n : u32) -> Self { ((self as $ U) >> n) as $ T } # [inline] fn swap_bytes (self) -> Self { <$ T >:: swap_bytes (self) } # [inline] fn reverse_bits (self) -> Self { <$ T >:: reverse_bits (self) } # [inline] fn from_be (x : Self) -> Self { <$ T >:: from_be (x) } # [inline] fn from_le (x : Self) -> Self { <$ T >:: from_le (x) } # [inline] fn to_be (self) -> Self { <$ T >:: to_be (self) } # [inline] fn to_le (self) -> Self { <$ T >:: to_le (self) } # [inline] fn pow (self , exp : u32) -> Self { <$ T >:: pow (self , exp) } } } ; }
};
}
