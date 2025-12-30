// Generated macro for impl_10480 (impl)
macro_rules! Depcrate_tuple_array_conversionsimpl_10480 {
() => {
// Module: crate::tuple_array_conversions
// Provides: {"impl_10480"}
// Dependencies: {}
impl PartialEq < PatKind < '_ > > for ToType { fn eq (& self , other : & PatKind < '_ >) -> bool { match self { ToType :: Array => matches ! (other , PatKind :: Tuple (_ , _)) , ToType :: Tuple => matches ! (other , PatKind :: Slice (_ , _ , _)) , } } }
};
}
