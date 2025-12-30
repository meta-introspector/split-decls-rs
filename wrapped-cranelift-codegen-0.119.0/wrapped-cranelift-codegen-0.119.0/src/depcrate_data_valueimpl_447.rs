// Generated macro for impl_447 (impl)
macro_rules! Depcrate_data_valueimpl_447 {
() => {
// Module: crate::data_value
// Provides: {"impl_447"}
// Dependencies: {}
impl Display for DataValue { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { DataValue :: I8 (dv) => write ! (f , "{dv}") , DataValue :: I16 (dv) => write ! (f , "{dv}") , DataValue :: I32 (dv) => write ! (f , "{dv}") , DataValue :: I64 (dv) => write ! (f , "{dv}") , DataValue :: I128 (dv) => write ! (f , "{dv}") , DataValue :: F16 (dv) => write ! (f , "{dv}") , DataValue :: F32 (dv) => write ! (f , "{dv}") , DataValue :: F64 (dv) => write ! (f , "{dv}") , DataValue :: F128 (dv) => write ! (f , "{dv}") , DataValue :: V128 (dv) => write ! (f , "{}" , ConstantData :: from (& dv [..])) , DataValue :: V64 (dv) => write ! (f , "{}" , ConstantData :: from (& dv [..])) , } } }
};
}
