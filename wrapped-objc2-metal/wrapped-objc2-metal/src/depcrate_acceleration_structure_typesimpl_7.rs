// Generated macro for impl_7 (impl)
macro_rules! Depcrate_acceleration_structure_typesimpl_7 {
() => {
// Module: crate::acceleration_structure_types
// Provides: {"impl_7"}
// Dependencies: {}
unsafe impl Encode for MTLPackedFloat3 { const ENCODING : Encoding = Encoding :: Struct ("_MTLPackedFloat3" , & [Encoding :: Union ("?" , & [Encoding :: Struct ("?" , & [c_float :: ENCODING , c_float :: ENCODING , c_float :: ENCODING] ,) , Encoding :: Array (3 , & c_float :: ENCODING) ,] ,)] ,) ; }
};
}
