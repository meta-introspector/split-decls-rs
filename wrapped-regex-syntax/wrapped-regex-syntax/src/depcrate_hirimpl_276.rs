// Generated macro for impl_276 (impl)
macro_rules! Depcrate_hirimpl_276 {
() => {
// Module: crate::hir
// Provides: {"impl_276"}
// Dependencies: {}
impl Interval for ClassBytesRange { type Bound = u8 ; # [inline] fn lower (& self) -> u8 { self . start } # [inline] fn upper (& self) -> u8 { self . end } # [inline] fn set_lower (& mut self , bound : u8) { self . start = bound ; } # [inline] fn set_upper (& mut self , bound : u8) { self . end = bound ; } # [doc = " Apply simple case folding to this byte range. Only ASCII case mappings"] # [doc = " (for a-z) are applied."] # [doc = ""] # [doc = " Additional ranges are appended to the given vector. Canonical ordering"] # [doc = " is *not* maintained in the given vector."] fn case_fold_simple (& self , ranges : & mut Vec < ClassBytesRange > ,) -> Result < () , unicode :: CaseFoldError > { if ! ClassBytesRange :: new (b'a' , b'z') . is_intersection_empty (self) { let lower = cmp :: max (self . start , b'a') ; let upper = cmp :: min (self . end , b'z') ; ranges . push (ClassBytesRange :: new (lower - 32 , upper - 32)) ; } if ! ClassBytesRange :: new (b'A' , b'Z') . is_intersection_empty (self) { let lower = cmp :: max (self . start , b'A') ; let upper = cmp :: min (self . end , b'Z') ; ranges . push (ClassBytesRange :: new (lower + 32 , upper + 32)) ; } Ok (()) } }
};
}
