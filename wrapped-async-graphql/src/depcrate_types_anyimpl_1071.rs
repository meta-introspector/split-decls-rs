// Generated macro for impl_1071 (impl)
macro_rules! Depcrate_types_anyimpl_1071 {
() => {
// Module: crate::types::any
// Provides: {"impl_1071"}
// Dependencies: {}
# [doc = " The `_Any` scalar is used to pass representations of entities from external"] # [doc = " services into the root `_entities` field for execution."] # [Scalar (internal , name = "_Any")] impl ScalarType for Any { fn parse (value : Value) -> InputValueResult < Self > { Ok (Self (value)) } fn is_valid (_value : & Value) -> bool { true } fn to_value (& self) -> Value { self . 0 . clone () } }
};
}
