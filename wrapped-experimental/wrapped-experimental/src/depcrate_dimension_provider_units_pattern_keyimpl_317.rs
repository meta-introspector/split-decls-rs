// Generated macro for impl_317 (impl)
macro_rules! Depcrate_dimension_provider_units_pattern_keyimpl_317 {
() => {
// Module: crate::dimension::provider::units::pattern_key
// Provides: {"impl_317"}
// Dependencies: {}
impl AsULE for PatternKey { type ULE = PatternKeyULE ; fn to_unaligned (self) -> Self :: ULE { let byte = match self { PatternKey :: Binary (value) => value , PatternKey :: Decimal (value) => { let sign = if value < 0 { 0b0010_0000 } else { 0 } ; debug_assert ! (value > - 32 && value < 32) ; (0b01 << 6) | sign | (value . unsigned_abs () & 0b0001_1111) } PatternKey :: Power { power , count } => { let power_bits = { match power { PowerValue :: Two => 0b10 << 4 , PowerValue :: Three => 0b11 << 4 , } } ; (0b10 << 6) | power_bits | count as u8 } } ; PatternKeyULE (byte) } fn from_unaligned (unaligned : Self :: ULE) -> Self { let byte = unaligned . 0 ; let variant = (byte & 0b1100_0000) >> 6 ; let value = byte & 0b0011_1111 ; match variant { 0b00 => PatternKey :: Binary (value) , 0b01 => match value & 0b0010_0000 { 0b0000_0000 => PatternKey :: Decimal (value as i8) , 0b0010_0000 => PatternKey :: Decimal (- ((value & 0b0001_1111) as i8)) , _ => unreachable ! () , } , 0b10 => { let power = match value & 0b0011_0000 { 0b0010_0000 => PowerValue :: Two , 0b0011_0000 => PowerValue :: Three , _ => unreachable ! () , } ; let count = value & 0b0000_1111 ; PatternKey :: Power { power , count : count . into () , } } _ => unreachable ! () , } } }
};
}
