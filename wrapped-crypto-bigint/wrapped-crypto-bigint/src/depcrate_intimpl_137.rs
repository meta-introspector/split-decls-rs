// Generated macro for impl_137 (impl)
macro_rules! Depcrate_intimpl_137 {
() => {
// Module: crate::int
// Provides: {"impl_137"}
// Dependencies: {}
impl < const LIMBS : usize > Signed for Int < LIMBS > { type Unsigned = Uint < LIMBS > ; fn abs_sign (& self) -> (Uint < LIMBS > , Choice) { let (abs , sign) = self . abs_sign () ; (abs , sign . into ()) } fn is_negative (& self) -> Choice { self . is_negative () . into () } fn is_positive (& self) -> Choice { self . is_positive () . into () } }
};
}
