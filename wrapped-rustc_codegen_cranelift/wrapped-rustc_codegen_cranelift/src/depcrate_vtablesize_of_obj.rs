// Generated macro for size_of_obj (function)
macro_rules! Depcrate_vtablesize_of_obj {
() => {
// Module: crate::vtable
// Provides: {"size_of_obj"}
// Dependencies: {}
pub (crate) fn size_of_obj (fx : & mut FunctionCx < '_ , '_ , '_ > , vtable : Value) -> Value { let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () as usize ; fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (ty :: COMMON_VTABLE_ENTRIES_SIZE * usize_size) as i32 ,) }
};
}
