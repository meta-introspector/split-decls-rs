// Generated macro for impl_34 (impl)
macro_rules! Depcrate_packedimpl_34 {
() => {
// Module: crate::packed
// Provides: {"impl_34"}
// Dependencies: {}
unsafe impl Encode for MPSPackedFloat3 { const ENCODING : Encoding = Encoding :: Struct ("_MPSPackedFloat3" , & [Encoding :: Union ("?" , & [Encoding :: Struct ("?" , & [c_float :: ENCODING , c_float :: ENCODING , c_float :: ENCODING] ,) , Encoding :: Array (3 , & c_float :: ENCODING) ,] ,)] ,) ; }
};
}
