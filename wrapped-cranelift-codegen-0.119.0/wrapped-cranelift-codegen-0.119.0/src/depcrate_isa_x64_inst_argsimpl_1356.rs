// Generated macro for impl_1356 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1356 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1356"}
// Dependencies: {}
impl CC { pub (crate) fn from_intcc (intcc : IntCC) -> Self { match intcc { IntCC :: Equal => CC :: Z , IntCC :: NotEqual => CC :: NZ , IntCC :: SignedGreaterThanOrEqual => CC :: NL , IntCC :: SignedGreaterThan => CC :: NLE , IntCC :: SignedLessThanOrEqual => CC :: LE , IntCC :: SignedLessThan => CC :: L , IntCC :: UnsignedGreaterThanOrEqual => CC :: NB , IntCC :: UnsignedGreaterThan => CC :: NBE , IntCC :: UnsignedLessThanOrEqual => CC :: BE , IntCC :: UnsignedLessThan => CC :: B , } } pub (crate) fn invert (& self) -> Self { match self { CC :: O => CC :: NO , CC :: NO => CC :: O , CC :: B => CC :: NB , CC :: NB => CC :: B , CC :: Z => CC :: NZ , CC :: NZ => CC :: Z , CC :: BE => CC :: NBE , CC :: NBE => CC :: BE , CC :: S => CC :: NS , CC :: NS => CC :: S , CC :: L => CC :: NL , CC :: NL => CC :: L , CC :: LE => CC :: NLE , CC :: NLE => CC :: LE , CC :: P => CC :: NP , CC :: NP => CC :: P , } } pub (crate) fn get_enc (self) -> u8 { self as u8 } }
};
}
