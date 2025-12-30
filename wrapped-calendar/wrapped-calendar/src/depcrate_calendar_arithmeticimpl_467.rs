// Generated macro for impl_467 (impl)
macro_rules! Depcrate_calendar_arithmeticimpl_467 {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"impl_467"}
// Dependencies: {}
impl < C : DateFieldsResolver > Hash for ArithmeticDate < C > { fn hash < H > (& self , state : & mut H) where H : Hasher , { self . year () . to_extended_year () . hash (state) ; self . month () . hash (state) ; self . day () . hash (state) ; } }
};
}
