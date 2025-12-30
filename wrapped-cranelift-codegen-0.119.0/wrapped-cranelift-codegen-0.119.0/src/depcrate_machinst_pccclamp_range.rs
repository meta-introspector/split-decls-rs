// Generated macro for clamp_range (function)
macro_rules! Depcrate_machinst_pccclamp_range {
() => {
// Module: crate::machinst::pcc
// Provides: {"clamp_range"}
// Dependencies: {}
pub (crate) fn clamp_range (ctx : & FactContext , to_bits : u16 , from_bits : u16 , fact : Option < Fact > ,) -> PccResult < Option < Fact > > { let max = if from_bits > 64 { return Ok (None) ; } else if from_bits == 64 { u64 :: MAX } else { (1u64 << from_bits) - 1 } ; trace ! ("clamp_range: fact {:?} from {} to {}" , fact , from_bits , to_bits) ; Ok (fact . and_then (| f | ctx . uextend (& f , from_bits , to_bits)) . or_else (| | { let result = Fact :: Range { bit_width : to_bits , min : 0 , max , } ; trace ! (" -> clamping to {:?}" , result) ; Some (result) })) }
};
}
