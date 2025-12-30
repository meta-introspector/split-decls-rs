// Generated macro for impl_269 (impl)
macro_rules! Depcrate_hirimpl_269 {
() => {
// Module: crate::hir
// Provides: {"impl_269"}
// Dependencies: {}
impl Interval for ClassUnicodeRange { type Bound = char ; # [inline] fn lower (& self) -> char { self . start } # [inline] fn upper (& self) -> char { self . end } # [inline] fn set_lower (& mut self , bound : char) { self . start = bound ; } # [inline] fn set_upper (& mut self , bound : char) { self . end = bound ; } # [doc = " Apply simple case folding to this Unicode scalar value range."] # [doc = ""] # [doc = " Additional ranges are appended to the given vector. Canonical ordering"] # [doc = " is *not* maintained in the given vector."] fn case_fold_simple (& self , ranges : & mut Vec < ClassUnicodeRange > ,) -> Result < () , unicode :: CaseFoldError > { let mut folder = unicode :: SimpleCaseFolder :: new () ? ; if ! folder . overlaps (self . start , self . end) { return Ok (()) ; } let (start , end) = (u32 :: from (self . start) , u32 :: from (self . end)) ; for cp in (start ..= end) . filter_map (char :: from_u32) { for & cp_folded in folder . mapping (cp) { ranges . push (ClassUnicodeRange :: new (cp_folded , cp_folded)) ; } } Ok (()) } }
};
}
