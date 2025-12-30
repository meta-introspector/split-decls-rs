// Generated macro for impl_130 (impl)
macro_rules! Depcrate_value_integerimpl_130 {
() => {
// Module: crate::value::integer
// Provides: {"impl_130"}
// Dependencies: {}
impl Integer { # [doc = " Returns the canonical length this integer will have when serialized to bytes."] # [doc = " This is called `canonical` as it is only used for canonically comparing two"] # [doc = " values. It shouldn't be used in any other context."] fn canonical_len (& self) -> usize { let x = self . 0 ; if let Ok (x) = u8 :: try_from (x) { if x < 24 { 1 } else { 2 } } else if let Ok (x) = i8 :: try_from (x) { if x >= - 24i8 { 1 } else { 2 } } else if u16 :: try_from (x) . is_ok () || i16 :: try_from (x) . is_ok () { 3 } else if u32 :: try_from (x) . is_ok () || i32 :: try_from (x) . is_ok () { 5 } else if u64 :: try_from (x) . is_ok () || i64 :: try_from (x) . is_ok () { 9 } else { x . to_be_bytes () . len () + 1 } } # [doc = " Compare two integers as if we were to serialize them, but more efficiently."] pub fn canonical_cmp (& self , other : & Self) -> Ordering { match self . canonical_len () . cmp (& other . canonical_len ()) { Ordering :: Equal => { match (self . 0 . is_negative () , other . 0 . is_negative ()) { (false , true) => Ordering :: Less , (true , false) => Ordering :: Greater , (true , true) => { match self . 0 . cmp (& other . 0) { Ordering :: Less => Ordering :: Greater , Ordering :: Equal => Ordering :: Equal , Ordering :: Greater => Ordering :: Less , } } (_ , _) => self . 0 . cmp (& other . 0) , } } x => x , } } }
};
}
