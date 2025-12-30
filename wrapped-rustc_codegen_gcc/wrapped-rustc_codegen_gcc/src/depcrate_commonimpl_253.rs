// Generated macro for impl_253 (impl)
macro_rules! Depcrate_commonimpl_253 {
() => {
// Module: crate::common
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { pub fn const_ptrcast (& self , val : RValue < 'gcc > , ty : Type < 'gcc >) -> RValue < 'gcc > { self . context . new_cast (None , val , ty) } pub fn const_bytes (& self , bytes : & [u8]) -> RValue < 'gcc > { bytes_in_context (self , bytes) } fn global_string (& self , string : & str) -> LValue < 'gcc > { let string = self . context . new_string_literal (string) ; let sym = self . generate_local_symbol_name ("str") ; let global = self . declare_private_global (& sym , self . val_ty (string)) ; global . global_set_initializer_rvalue (string) ; global } pub fn const_bitcast (& self , value : RValue < 'gcc > , typ : Type < 'gcc >) -> RValue < 'gcc > { if value . get_type () == self . bool_type . make_pointer () && let Some (pointee) = typ . get_pointee () && pointee . dyncast_vector () . is_some () { panic ! () } self . bitcast_if_needed (value , typ) } }
};
}
