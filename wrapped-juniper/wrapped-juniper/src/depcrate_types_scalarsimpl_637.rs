// Generated macro for impl_637 (impl)
macro_rules! Depcrate_types_scalarsimpl_637 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_637"}
// Dependencies: {}
impl < S > GraphQLValue < S > for str where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType < S > > :: name (info) } fn resolve (& self , _ : & () , _ : Option < & [Selection < S >] > , _ : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { Ok (Value :: Scalar (self . to_scalar_value ())) } }
};
}
