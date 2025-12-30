// Generated macro for impl_42 (impl)
macro_rules! Depcrate_patternsimpl_42 {
() => {
// Module: crate::patterns
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > ConditionalListJoinerPattern < 'a > { pub (crate) fn parts < 'b , W : Writeable + ? Sized > (& 'a self , following_value : & 'b W ,) -> PatternParts < 'a > { match & self . special_case { Some (SpecialCasePattern { condition , pattern }) if condition . deref () . matches_earliest_fwd_lazy (following_value) => { pattern . parts () } _ => self . default . parts () , } } # [doc = " The expected length of this pattern"] fn size_hint (& 'a self) -> LengthHint { let mut hint = self . default . size_hint () ; if let Some (special_case) = & self . special_case { hint |= special_case . pattern . size_hint () } hint } }
};
}
