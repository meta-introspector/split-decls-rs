// Generated macro for IntTypeBounds (trait)
macro_rules! Depcrate_constsIntTypeBounds {
() => {
// Module: crate::consts
// Provides: {"IntTypeBounds"}
// Dependencies: {}
trait IntTypeBounds : Sized { type Output : PartialOrd ; fn min_max (self) -> Option < (Self :: Output , Self :: Output) > ; fn bits (self) -> Self :: Output ; fn ensure_fits (self , val : Self :: Output) -> Option < Self :: Output > { let (min , max) = self . min_max () ? ; (min <= val && val <= max) . then_some (val) } }
};
}
