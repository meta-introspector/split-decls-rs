// Generated macro for ScalarType (trait)
macro_rules! Depcrate_resolver_utils_scalarScalarType {
() => {
// Module: crate::resolver_utils::scalar
// Provides: {"ScalarType"}
// Dependencies: {}
# [doc = " A GraphQL scalar."] # [doc = ""] # [doc = " You can implement the trait to create a custom scalar."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " struct MyInt(i32);"] # [doc = ""] # [doc = " #[Scalar]"] # [doc = " impl ScalarType for MyInt {"] # [doc = "     fn parse(value: Value) -> InputValueResult<Self> {"] # [doc = "         if let Value::Number(n) = &value {"] # [doc = "             if let Some(n) = n.as_i64() {"] # [doc = "                 return Ok(MyInt(n as i32));"] # [doc = "             }"] # [doc = "         }"] # [doc = "         Err(InputValueError::expected_type(value))"] # [doc = "     }"] # [doc = ""] # [doc = "     fn to_value(&self) -> Value {"] # [doc = "         Value::Number(self.0.into())"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait ScalarType : Sized + Send { # [doc = " Parse a scalar value."] fn parse (value : Value) -> InputValueResult < Self > ; # [doc = " Checks for a valid scalar value."] # [doc = ""] # [doc = " Implementing this function can find incorrect input values during the"] # [doc = " verification phase, which can improve performance."] fn is_valid (_value : & Value) -> bool { true } # [doc = " Convert the scalar to `Value`."] fn to_value (& self) -> Value ; }
};
}
