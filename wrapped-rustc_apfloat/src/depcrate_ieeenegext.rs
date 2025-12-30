// Generated macro for NegExt (trait)
macro_rules! Depcrate_ieeeNegExt {
() => {
// Module: crate::ieee
// Provides: {"NegExt"}
// Dependencies: {}
trait NegExt : Neg < Output = Self > + Sized { fn negate_if (self , negate : bool) -> Self { if negate { - self } else { self } } fn with_sign (self , sign : bool) -> Self where Self : Float , { self . negate_if (self . is_negative () != sign) } }
};
}
