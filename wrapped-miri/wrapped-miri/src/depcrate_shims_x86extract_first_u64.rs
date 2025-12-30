// Generated macro for extract_first_u64 (function)
macro_rules! Depcrate_shims_x86extract_first_u64 {
() => {
// Module: crate::shims::x86
// Provides: {"extract_first_u64"}
// Dependencies: {}
# [doc = " Takes a 128-bit vector, transmutes it to `[u64; 2]` and extracts"] # [doc = " the first value."] fn extract_first_u64 < 'tcx > (ecx : & crate :: MiriInterpCx < 'tcx > , op : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , u64 > { let array_layout = ecx . layout_of (Ty :: new_array (ecx . tcx . tcx , ecx . tcx . types . u64 , 2)) ? ; let op = op . transmute (array_layout , ecx) ? ; ecx . read_scalar (& ecx . project_index (& op , 0) ?) ? . to_u64 () }
};
}
