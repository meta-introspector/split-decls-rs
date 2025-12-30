// Generated macro for impl_429 (impl)
macro_rules! Depcrate_data_valueimpl_429 {
() => {
// Module: crate::data_value
// Provides: {"impl_429"}
// Dependencies: {}
impl PartialEq for DataValue { fn eq (& self , other : & Self) -> bool { use DataValue :: * ; match (self , other) { (I8 (l) , I8 (r)) => l == r , (I8 (_) , _) => false , (I16 (l) , I16 (r)) => l == r , (I16 (_) , _) => false , (I32 (l) , I32 (r)) => l == r , (I32 (_) , _) => false , (I64 (l) , I64 (r)) => l == r , (I64 (_) , _) => false , (I128 (l) , I128 (r)) => l == r , (I128 (_) , _) => false , (F16 (l) , F16 (r)) => l . partial_cmp (& r) == Some (Ordering :: Equal) , (F16 (_) , _) => false , (F32 (l) , F32 (r)) => l . as_f32 () == r . as_f32 () , (F32 (_) , _) => false , (F64 (l) , F64 (r)) => l . as_f64 () == r . as_f64 () , (F64 (_) , _) => false , (F128 (l) , F128 (r)) => l . partial_cmp (& r) == Some (Ordering :: Equal) , (F128 (_) , _) => false , (V128 (l) , V128 (r)) => l == r , (V128 (_) , _) => false , (V64 (l) , V64 (r)) => l == r , (V64 (_) , _) => false , } } }
};
}
