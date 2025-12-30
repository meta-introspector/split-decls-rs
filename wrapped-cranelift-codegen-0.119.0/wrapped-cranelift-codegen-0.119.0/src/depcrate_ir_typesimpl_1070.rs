// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_ir_typesimpl_1070 {
() => {
// Module: crate::ir::types
// Provides: {"impl_1070"}
// Dependencies: {}
impl Debug for Type { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { if self . is_int () { write ! (f , "types::I{}" , self . lane_bits ()) } else if self . is_float () { write ! (f , "types::F{}" , self . lane_bits ()) } else if self . is_vector () { write ! (f , "{:?}X{}" , self . lane_type () , self . lane_count ()) } else if self . is_dynamic_vector () { write ! (f , "{:?}X{}XN" , self . lane_type () , self . min_lane_count ()) } else { match * self { INVALID => write ! (f , "types::INVALID") , _ => write ! (f , "Type(0x{:x})" , self . 0) , } } } }
};
}
