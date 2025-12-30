// Generated macro for impl_343 (impl)
macro_rules! Depcrate_debuginfoimpl_343 {
() => {
// Module: crate::debuginfo
// Provides: {"impl_343"}
// Dependencies: {}
impl < 'a , 'gcc , 'tcx > DebugInfoBuilderMethods for Builder < 'a , 'gcc , 'tcx > { fn dbg_var_addr (& mut self , _dbg_var : Self :: DIVariable , _dbg_loc : Self :: DILocation , _variable_alloca : Self :: Value , _direct_offset : Size , _indirect_offsets : & [Size] , _fragment : Option < Range < Size > > ,) { # [cfg (feature = "master")] _variable_alloca . set_location (_dbg_loc) ; } fn insert_reference_to_gdb_debug_scripts_section_global (& mut self) { } # [doc = " FIXME(tempdragon): Currently, this function is not yet implemented. It seems that the"] # [doc = " debug name and the mangled name should both be included in the LValues."] # [doc = " Besides, a function to get the rvalue type(m_is_lvalue) should also be included."] fn set_var_name (& mut self , _value : RValue < 'gcc > , _name : & str) { } fn set_dbg_loc (& mut self , dbg_loc : Self :: DILocation) { self . location = Some (dbg_loc) ; } fn clear_dbg_loc (& mut self) { self . location = None ; } }
};
}
