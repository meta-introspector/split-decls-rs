// Generated macro for lower_condcode (function)
macro_rules! Depcrate_isa_aarch64_lowerlower_condcode {
() => {
// Module: crate::isa::aarch64::lower
// Provides: {"lower_condcode"}
// Dependencies: {}
pub (crate) fn lower_condcode (cc : IntCC) -> Cond { match cc { IntCC :: Equal => Cond :: Eq , IntCC :: NotEqual => Cond :: Ne , IntCC :: SignedGreaterThanOrEqual => Cond :: Ge , IntCC :: SignedGreaterThan => Cond :: Gt , IntCC :: SignedLessThanOrEqual => Cond :: Le , IntCC :: SignedLessThan => Cond :: Lt , IntCC :: UnsignedGreaterThanOrEqual => Cond :: Hs , IntCC :: UnsignedGreaterThan => Cond :: Hi , IntCC :: UnsignedLessThanOrEqual => Cond :: Ls , IntCC :: UnsignedLessThan => Cond :: Lo , } }
};
}
