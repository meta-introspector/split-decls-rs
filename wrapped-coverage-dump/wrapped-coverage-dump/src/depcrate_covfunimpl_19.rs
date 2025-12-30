// Generated macro for impl_19 (impl)
macro_rules! Depcrate_covfunimpl_19 {
() => {
// Module: crate::covfun
// Provides: {"impl_19"}
// Dependencies: {}
impl CovTerm { pub (crate) fn decode (input : u32) -> Option < Self > { let (high , tag) = (input >> 2 , input & 0b11) ; match tag { 0b00 if high == 0 => Some (Self :: Zero) , 0b01 => Some (Self :: Counter (high)) , 0b10 => Some (Self :: Expression (high , Op :: Sub)) , 0b11 => Some (Self :: Expression (high , Op :: Add)) , _ => None , } } }
};
}
