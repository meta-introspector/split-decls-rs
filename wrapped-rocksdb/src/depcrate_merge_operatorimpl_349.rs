// Generated macro for impl_349 (impl)
macro_rules! Depcrate_merge_operatorimpl_349 {
() => {
// Module: crate::merge_operator
// Provides: {"impl_349"}
// Dependencies: {}
impl MergeOperands { fn new (operands_list : * const * const c_char , operands_list_len : * const size_t , num_operands : c_int ,) -> MergeOperands { assert ! (num_operands >= 0) ; MergeOperands { operands_list , operands_list_len , num_operands : num_operands as usize , } } pub fn len (& self) -> usize { self . num_operands } pub fn is_empty (& self) -> bool { self . num_operands == 0 } pub fn iter (& self) -> MergeOperandsIter < '_ > { MergeOperandsIter { operands : self , cursor : 0 , } } fn get_operand (& self , index : usize) -> Option < & [u8] > { if index >= self . num_operands { None } else { unsafe { let base = self . operands_list as usize ; let base_len = self . operands_list_len as usize ; let spacing = mem :: size_of :: < * const * const u8 > () ; let spacing_len = mem :: size_of :: < * const size_t > () ; let len_ptr = (base_len + (spacing_len * index)) as * const size_t ; let len = * len_ptr ; let ptr = base + (spacing * index) ; Some (slice :: from_raw_parts (* (ptr as * const * const u8) , len)) } } } }
};
}
