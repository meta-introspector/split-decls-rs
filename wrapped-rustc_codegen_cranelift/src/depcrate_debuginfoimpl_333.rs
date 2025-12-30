// Generated macro for impl_333 (impl)
macro_rules! Depcrate_debuginfoimpl_333 {
() => {
// Module: crate::debuginfo
// Provides: {"impl_333"}
// Dependencies: {}
impl FunctionDebugContext { pub (crate) fn finalize (mut self , debug_context : & mut DebugContext , func_id : FuncId , context : & Context ,) { let end = self . create_debug_lines (debug_context , func_id , context) ; debug_context . unit_range_list . 0 . push (Range :: StartLength { begin : address_for_func (func_id) , length : u64 :: from (end) }) ; let func_entry = debug_context . dwarf . unit . get_mut (self . entry_id) ; func_entry . set (gimli :: DW_AT_low_pc , AttributeValue :: Address (address_for_func (func_id))) ; func_entry . set (gimli :: DW_AT_high_pc , AttributeValue :: Udata (u64 :: from (end))) ; } }
};
}
