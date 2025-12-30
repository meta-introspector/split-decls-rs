// Generated macro for exit_input_value (function)
macro_rules! Depcrate_validation_visitorexit_input_value {
() => {
// Module: crate::validation::visitor
// Provides: {"exit_input_value"}
// Dependencies: {}
fn exit_input_value < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , input_value : & 'a Spanning < InputValue < S > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { use crate :: InputValue :: * ; let span = & input_value . span ; match & input_value . item { Null => v . exit_null_value (ctx , Spanning { span , item : & () }) , Scalar (item) => v . exit_scalar_value (ctx , Spanning { span , item }) , Enum (item) => v . exit_enum_value (ctx , Spanning { span , item }) , Variable (item) => v . exit_variable_value (ctx , Spanning { span , item }) , List (item) => v . exit_list_value (ctx , Spanning { span , item }) , Object (item) => v . exit_object_value (ctx , Spanning { span , item }) , } }
};
}
