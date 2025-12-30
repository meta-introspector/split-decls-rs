// Generated macro for align_of_obj (function)
macro_rules! Depcrate_vtablealign_of_obj {
() => {
// Module: crate::vtable
// Provides: {"align_of_obj"}
// Dependencies: {}
pub (crate) fn align_of_obj (fx : & mut FunctionCx < '_ , '_ , '_ > , vtable : Value) -> Value { let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () as usize ; fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (ty :: COMMON_VTABLE_ENTRIES_ALIGN * usize_size) as i32 ,) }
};
}
