// Generated macro for get_max_bitfield_range (function)
macro_rules! Depcrateget_max_bitfield_range {
() => {
// Module: crate
// Provides: {"get_max_bitfield_range"}
// Dependencies: {}
# [doc = " Returns `Some(smallest_bit_index, largest_bit_index)` contained in `params` if"] # [doc = " `params` contains any bitfields. Otherwise `None`."] pub fn get_max_bitfield_range < 'a , I > (params : I) -> Option < (u8 , u8) > where I : Iterator < Item = & 'a Parameter > + Clone , { let largest_bit_index = params . clone () . map (| param | match & param . ty { Type :: BitField (range) => range . end , _ => unreachable ! () , }) . max () ; let smallest_bit_index = params . map (| param | match & param . ty { Type :: BitField (range) => range . start , _ => unreachable ! () , }) . min () ; match (smallest_bit_index , largest_bit_index) { (Some (smallest) , Some (largest)) => Some ((smallest , largest)) , (None , None) => None , _ => unreachable ! () , } }
};
}
