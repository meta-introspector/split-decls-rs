// Generated macro for impl_1069 (impl)
macro_rules! Depcrate_ir_typesimpl_1069 {
() => {
// Module: crate::ir::types
// Provides: {"impl_1069"}
// Dependencies: {}
impl Display for Type { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { if self . is_int () { write ! (f , "i{}" , self . lane_bits ()) } else if self . is_float () { write ! (f , "f{}" , self . lane_bits ()) } else if self . is_vector () { write ! (f , "{}x{}" , self . lane_type () , self . lane_count ()) } else if self . is_dynamic_vector () { write ! (f , "{:?}x{}xN" , self . lane_type () , self . min_lane_count ()) } else { match * self { INVALID => panic ! ("INVALID encountered") , _ => panic ! ("Unknown Type(0x{:x})" , self . 0) , } } } }
};
}
