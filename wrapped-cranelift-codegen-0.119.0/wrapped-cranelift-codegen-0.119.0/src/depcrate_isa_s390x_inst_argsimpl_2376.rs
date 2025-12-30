// Generated macro for impl_2376 (impl)
macro_rules! Depcrate_isa_s390x_inst_argsimpl_2376 {
() => {
// Module: crate::isa::s390x::inst::args
// Provides: {"impl_2376"}
// Dependencies: {}
impl Cond { pub fn from_mask (mask : u8) -> Cond { assert ! (mask >= 1 && mask <= 14) ; Cond { mask } } pub fn from_intcc (cc : IntCC) -> Cond { let mask = match cc { IntCC :: Equal => 8 , IntCC :: NotEqual => 4 | 2 , IntCC :: SignedGreaterThanOrEqual => 8 | 2 , IntCC :: SignedGreaterThan => 2 , IntCC :: SignedLessThanOrEqual => 8 | 4 , IntCC :: SignedLessThan => 4 , IntCC :: UnsignedGreaterThanOrEqual => 8 | 2 , IntCC :: UnsignedGreaterThan => 2 , IntCC :: UnsignedLessThanOrEqual => 8 | 4 , IntCC :: UnsignedLessThan => 4 , } ; Cond { mask } } pub fn from_floatcc (cc : FloatCC) -> Cond { let mask = match cc { FloatCC :: Ordered => 8 | 4 | 2 , FloatCC :: Unordered => 1 , FloatCC :: Equal => 8 , FloatCC :: NotEqual => 4 | 2 | 1 , FloatCC :: OrderedNotEqual => 4 | 2 , FloatCC :: UnorderedOrEqual => 8 | 1 , FloatCC :: LessThan => 4 , FloatCC :: LessThanOrEqual => 8 | 4 , FloatCC :: GreaterThan => 2 , FloatCC :: GreaterThanOrEqual => 8 | 2 , FloatCC :: UnorderedOrLessThan => 4 | 1 , FloatCC :: UnorderedOrLessThanOrEqual => 8 | 4 | 1 , FloatCC :: UnorderedOrGreaterThan => 2 | 1 , FloatCC :: UnorderedOrGreaterThanOrEqual => 8 | 2 | 1 , } ; Cond { mask } } # [doc = " Return the inverted condition."] pub fn invert (self) -> Cond { Cond { mask : ! self . mask & 15 , } } # [doc = " Return the machine encoding of this condition."] pub fn bits (self) -> u8 { self . mask } }
};
}
